import pygame
import math
from pygame import gfxdraw
import numpy

# screen variables
screenWidth = 1280
screenHeight = 720

# simulation variables
simSpeed = 60

# camera variables
cameraPos = (0,0,1)
cameraAngle = (1,1,0)
FOV = 90
cameraPanSpeed = 0.02
cameraMoveSpeed = 0.1

# object variables
objects = []

# main object(s)
points = [(5,5,0), (5,6,0), (6,5,0), (5,5,2)]
connections = [(0,1), (0,2), (0,3), (1,2), (1,3), (2,3)]
color = (0,255,0)
drawPoints = True
objects.append((points, connections, color, drawPoints))

points = [(4,6,0), (4,5,0), (3,6,0), (3,5,0), (4,6,0.75), (4,5,0.75), (3,6,0.75), (3,5,0.75)]
connections = [(0,1), (0,2), (3,1), (3,2), (4,5), (4,6), (7,5), (7,6), (0,4), (1,5), (2,6), (3,7)]
color = (255,0,0)
drawPoints = True
objects.append((points, connections, color, drawPoints))

points = [(6,4,0), (5,4,0), (6,3,0), (5,3,0), (6,4,1.25), (6,3,1.25)]
connections = [(0,1), (0,2), (3,1), (3,2), (4,5), (0,4), (2,5), (1,4), (3,5)]
color = (0,255,255)
drawPoints = True
objects.append((points, connections, color, drawPoints))



# horizontal plane
points = []
connections = []
color = (255,255,255)
drawPoints = False

numberOfPlaneLines = 50
spacingOfPlaneLines = 1
for i in range(numberOfPlaneLines // 2):
    points.append((numberOfPlaneLines // 2 * spacingOfPlaneLines, i * spacingOfPlaneLines, 0))
    points.append((-numberOfPlaneLines // 2 * spacingOfPlaneLines, i * spacingOfPlaneLines, 0))
    connections.append((8*i, 8*i+1))
    points.append((numberOfPlaneLines // 2 * spacingOfPlaneLines, -i * spacingOfPlaneLines, 0))
    points.append((-numberOfPlaneLines // 2 * spacingOfPlaneLines, -i * spacingOfPlaneLines, 0))
    connections.append((8*i+2, 8*i+3))
    points.append((i * spacingOfPlaneLines, numberOfPlaneLines // 2 * spacingOfPlaneLines, 0))
    points.append((i * spacingOfPlaneLines, -numberOfPlaneLines // 2 * spacingOfPlaneLines, 0))
    connections.append((8*i+4, 8*i+5))
    points.append((-i * spacingOfPlaneLines, numberOfPlaneLines // 2 * spacingOfPlaneLines, 0))
    points.append((-i * spacingOfPlaneLines, -numberOfPlaneLines // 2 * spacingOfPlaneLines, 0))
    connections.append((8*i+6, 8*i+7))
objects.append((points, connections, color, drawPoints))

# extra object (for diagnostics)
points = [(10,10,0), (10,15,0)]
connections = [(0,1)]
drawPoints = True
color = (255,0,0)
# objects.append((points, connections, color, drawPoints))

# processed variables
horizontalRotation = math.atan(cameraAngle[1] / cameraAngle[0])
verticalRotation = 0
FOV = FOV / 180 * math.pi
horizontalRotationSpeed = 0
verticalRotationSpeed = 0
cameraTranslation = (0,0,0)
cameraAngle = (cameraAngle[0] + 0.0001, cameraAngle[1] + 0.0001, cameraAngle[2] + 0.0001)
cameraPos = (cameraPos[0] + 0.0001, cameraPos[1] + 0.0001, cameraPos[2] + 0.0001)


def findCenterOfPlane(cameraPos, cameraAngle):
    # Let a, b, c represent vector components of camera angle
    a, b, c = cameraAngle[0], cameraAngle[1], cameraAngle[2]

    # Let x, y, z represent 3d coordinates of camera pos
    x, y, z = cameraPos[0], cameraPos[1], cameraPos[2]

    magnitudeOfCameraPos = magnitude = math.sqrt(a**2 + b**2 + c**2)

    # Let x2, y2, z2 represent 3d coordinates of center of plane
    x2 = x + a/magnitude
    y2 = y + b/magnitude
    z2 = z + c/magnitude
    return (x2, y2, z2)

def findEquationOfPlane(cameraAngle, centerOfPlane):
    # Let a, b, c represent vector components of camera angle
    a, b, c = cameraAngle[0], cameraAngle[1], cameraAngle[2]

    # Let x, y, z represent 3d coordinates of center of plane
    x, y, z = centerOfPlane[0], centerOfPlane[1], centerOfPlane[2]

    d = -(a*x) - b*y - c*z

    return (a, b, c, d)

def findIntersectionOnPlane(cameraPos, pointPos, equationOfPlane):
    # Let x1, y1, z1 = point pos coordinates
    x1, y1, z1 = pointPos[0], pointPos[1], pointPos[2]

    # Let x2, y2, z2 = camera pos coordinates
    x2, y2, z2, = cameraPos[0], cameraPos[1], cameraPos[2]

    # Let p, q, r = direction vector from pointPos to cameraPos (direction of pointPos vector)
    p, q, r = x1 - x2, y1 - y2, z1 - z2

    # Let a, b, c, d = coefficients and constants of the equation of the plane
    a, b, c, d = equationOfPlane[0], equationOfPlane[1], equationOfPlane[2], equationOfPlane[3]

    LAMBDA = - (a*x1 + b*y1 + c*z1 + d) / (a*p + b*q + c*r)

    # Let x3, y3, z3 = coordinates of intersection on plane
    x3, y3, z3 = x1 + LAMBDA*p, y1 + LAMBDA*q, z1 + LAMBDA*r

    return (x3, y3, z3)

def findClosestPointOnHorizon(cameraAngle, centerOfPlane, intersectionOnPlane):
    # Let p, q = horizontal components (x and y) of camera angle
    p, q = cameraAngle[0], cameraAngle[1]

    # Let x1, y1, z1 = coordinates of center of plane
    x1, y1, z1 = centerOfPlane[0], centerOfPlane[1], centerOfPlane[2]

    # Let x2, y2, z2 = coordinates of intersection on plane (intersection of point vector with plane)
    x2, y2, z2 = intersectionOnPlane[0], intersectionOnPlane[1], intersectionOnPlane[2]

    LAMBDA = (q*x1 - q*x2 + p*y2 - p*y1) / (q**2 + p**2)

    # Let x3, y3, z3 = coordinates of closest point on horizon line
    x3, y3, z3 = x1 - LAMBDA*q, y1 + LAMBDA*p, z1

    if LAMBDA > 0:
        direction = True
    else:
        direction = False


    return (x3, y3, z3, direction)

def findScreenComponentDistances(centerOfPlane, intersectionOnPlane, closestPointOnHorizon):
    horizontalDistanceOnScreen = findDistance(centerOfPlane, closestPointOnHorizon)
    verticalDistanceOnScreen = findDistance(closestPointOnHorizon, intersectionOnPlane)

    if intersectionOnPlane[2] < closestPointOnHorizon[2]:
        verticalDistanceOnScreen *= -1

    if closestPointOnHorizon[3]: #based on sign for lambda found before
        horizontalDistanceOnScreen *= -1

    return (horizontalDistanceOnScreen, verticalDistanceOnScreen)

def findDisplayDistances(screenComponentDistances, FOV, screenWidth):
    x, y = screenComponentDistances[0], screenComponentDistances[1]

    maxDistOnScreenForFOV = math.tan(FOV/2)

    xDisplayDistance = x / maxDistOnScreenForFOV * screenWidth/2
    yDisplayDistance = y / maxDistOnScreenForFOV * screenWidth/2

    return xDisplayDistance, yDisplayDistance


def findDistance(p1, p2):
    # Basic formula to find distance between 2 points
    return math.sqrt((p1[0] - p2[0])**2 + (p1[1] - p2[1])**2 + (p1[2] - p2[2])**2)

def findHorizontalAngle(p1, p2):
    # Establishing horizontal coordinates
    x1, y1 = p1[0], p1[1]
    x2, y2 = p2[0], p2[1]

    slope = (y2 - y1) / (x2 - x1)

    angle = math.atan(slope)
    if x2 < x1 and y2 > y1:
        angle += math.pi
    if x2 < x1 and y2 <= y1:
        angle -= math.pi

    return angle

def findAngleBetweenVectors(v, w):
    # Establishing the 3 components of each vector
    v1, v2, v3 = v[0], v[1], v[2]
    w1, w2, w3 = w[0], w[1], w[2]

    magnitudeV = math.sqrt(v1**2 + v2**2 + v3**2)
    magnitudeW = math.sqrt(w1**2 + w2**2 + w3**2)

    angle = math.acos((v1*w1 + v2*w2 + v3*w3) / (magnitudeV * magnitudeW))
    return angle

def modulateHorizontalComponents(vector):
    p, q = vector[0], vector[1]
    magnitude = math.sqrt(p**2 + q**2)
    p = p / magnitude
    q = q / magnitude

    return (p, q, vector[2])


# Example file showing a basic pygame "game loop"

# pygame setup
pygame.init()
screen = pygame.display.set_mode((screenWidth, screenHeight))
clock = pygame.time.Clock()
running = True

while running:
    # poll for events
    # pygame.QUIT event means the user clicked X to close your window
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

        # Keydown events, creates camera movement/rotation speed
        if event.type == pygame.KEYDOWN:
            if event.key == pygame.K_LEFT:
                horizontalRotationSpeed = cameraPanSpeed
            if event.key == pygame.K_RIGHT:
                horizontalRotationSpeed = -cameraPanSpeed
            if event.key == pygame.K_UP:
                verticalRotationSpeed = cameraPanSpeed
            if event.key == pygame.K_DOWN:
                verticalRotationSpeed = -cameraPanSpeed
            if event.key == pygame.K_w:
                cameraTranslation = (cameraMoveSpeed, cameraTranslation[1], 0)
            if event.key == pygame.K_s:
                cameraTranslation = (-cameraMoveSpeed, cameraTranslation[1], 0)
            if event.key == pygame.K_a:
                cameraTranslation = (cameraTranslation[0], cameraMoveSpeed, 0)
                # cameraPos = (cameraPos[0] - cameraAngle[1], cameraPos[1] + cameraAngle[0], cameraPos[2])
            if event.key == pygame.K_d:
                cameraTranslation = (cameraTranslation[0], -cameraMoveSpeed, 0)
                # cameraPos = (cameraPos[0] + cameraAngle[1], cameraPos[1] - cameraAngle[0], cameraPos[2])

        # Keyup events, removes camera movement/rotation speeds
        if event.type == pygame.KEYUP:
            if event.key == pygame.K_LEFT:
                horizontalRotationSpeed = 0
            if event.key == pygame.K_RIGHT:
                horizontalRotationSpeed = 0
            if event.key == pygame.K_UP:
                verticalRotationSpeed = 0
            if event.key == pygame.K_DOWN:
                verticalRotationSpeed = 0
            if event.key == pygame.K_w:
                cameraTranslation = (0, cameraTranslation[1], 0)
            if event.key == pygame.K_s:
                cameraTranslation = (0, cameraTranslation[1], 0)
            if event.key == pygame.K_a:
                cameraTranslation = (cameraTranslation[0], 0, 0)
            if event.key == pygame.K_d:
                cameraTranslation = (cameraTranslation[0], 0, 0)


    # CAMERA MOTION
    # Camera rotation
    horizontalRotation += horizontalRotationSpeed
    verticalRotation += verticalRotationSpeed
    if abs(verticalRotation) > math.pi/2 - 0.01: # This stops vertical rotation at 90 degrees
        verticalRotation = numpy.sign(verticalRotation) * (math.pi/2 - 0.01)
    cameraAngle = (math.cos(horizontalRotation), math.sin(horizontalRotation), math.tan(verticalRotation))
    cameraAngle = modulateHorizontalComponents(cameraAngle)

    # Camera translation
    if cameraTranslation[0] != 0 or cameraTranslation[1] != 0: # modulate camera translation so magnitude is always 1
        cameraTranslation = modulateHorizontalComponents(cameraTranslation)

    cameraTranslation = (cameraMoveSpeed * cameraTranslation[0], cameraMoveSpeed * cameraTranslation[1], 0)
    cameraPos = (cameraPos[0] + cameraAngle[0] * cameraTranslation[0] - cameraAngle[1] * cameraTranslation[1], cameraPos[1] + cameraAngle[1] * cameraTranslation[0] + cameraAngle[0] * cameraTranslation[1], cameraPos[2])


    # fill the screen with a color to wipe away anything from last frame
    screen.fill("black")

    # GAME UPDATES

    # values needed for every frame
    centerOfPlane = findCenterOfPlane(cameraPos, cameraAngle)
    equationOfPlane = findEquationOfPlane(cameraAngle, centerOfPlane)

    pointsOnDisplay = [] # [[listOfPoints],isBehind]
    for j in range(len(objects)): # iterates through each object
        object = objects[j]
        points = object[0]
        connections = object[1]
        color = object[2]
        drawPoints = object[3]
        pointsOnDisplay = []

        for i in range(len(points)): # iterates through all points in a given object
            pointPos = points[i]
            intersectionOnPlane = findIntersectionOnPlane(cameraPos, pointPos, equationOfPlane)
            closestPointOnHorizon = findClosestPointOnHorizon(cameraAngle, centerOfPlane, intersectionOnPlane)
            screenComponentDistances = findScreenComponentDistances(centerOfPlane, intersectionOnPlane, closestPointOnHorizon)
            displayDistances = findDisplayDistances(screenComponentDistances, FOV, screenWidth)

            isBehind = False
            if findAngleBetweenVectors(cameraAngle, (pointPos[0] - cameraPos[0], pointPos[1] - cameraPos[1], pointPos[2] - cameraPos[2])) >= math.pi/2:
                isBehind = True

            # ensures points are within renderable distance (even if offscreen), scales down proportionally to 30k max
            if (abs(displayDistances[1]) > 30000):
                displayDistances = (displayDistances[0] * numpy.sign(displayDistances[1])*30000 / displayDistances[1], numpy.sign(displayDistances[1])*30000)
            if (abs(displayDistances[0]) > 30000):
                displayDistances = (numpy.sign(displayDistances[0])*30000, displayDistances[1] * numpy.sign(displayDistances[0])*30000 / displayDistances[0])

            pointsOnDisplay.append((int(screenWidth / 2 + displayDistances[0]), int(screenHeight / 2 - displayDistances[1]), isBehind))

        # drawing points
        for i in range(len(pointsOnDisplay)):
            if drawPoints:
                if pointsOnDisplay[i][2] == True:
                    continue
                pygame.gfxdraw.filled_circle(screen, pointsOnDisplay[i][0], pointsOnDisplay[i][1], 4, color)
            else:
                break

        # drawing lines
        for i in range(len(connections)):
            p1 = pointsOnDisplay[connections[i][0]]
            p2 = pointsOnDisplay[connections[i][1]]
            if p1[2] == True and p2[2] == True:
                continue
            if p1[2] == True: # if one of the points is out of view (MAGIC)
                deltaX = p2[0] - p1[0]
                deltaY = p2[1] - p1[1]
                d1 = (numpy.sign(deltaX) * screenWidth - p1[0]) / deltaX
                d2 = (numpy.sign(deltaY) * screenWidth - p1[1]) / deltaY
                if d1 < d2:
                    p1 = (p1[0] + d1 * deltaX, p1[1] + d1 * deltaY, p1[2])
                else:
                    p1 = (p1[0] + d2 * deltaX, p1[1] + d2 * deltaY, p1[2])
            if p2[2] == True: # if the other point is out of view (MAGIC)
                deltaX = p1[0] - p2[0]
                deltaY = p1[1] - p2[1]
                d1 = (numpy.sign(deltaX) * screenWidth - p2[0]) / deltaX
                d2 = (numpy.sign(deltaY) * screenWidth - p2[1]) / deltaY
                if d1 < d2:
                    p2 = (p2[0] + d1 * deltaX, p2[1] + d1 * deltaY, p2[2])
                else:
                    p2 = (p2[0] + d2 * deltaX, p2[1] + d2 * deltaY, p2[2])

            pygame.draw.aaline(screen, color, (p1[0], p1[1]), (p2[0], p2[1]))


    # diagnostics

    # flip() the display to put your work on screen
    pygame.display.flip()
    clock.tick(simSpeed)  # limits FPS to 60

pygame.quit()