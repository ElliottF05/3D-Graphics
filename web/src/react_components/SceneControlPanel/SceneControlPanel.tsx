import * as wasm from "@wasm/wasm_graphics"

import React, { useEffect, useState } from 'react';
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
} from "@/components/ui/accordion";
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from "@/components/ui/tooltip";
import { Slider } from "@/components/ui/slider";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch"
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";

import EditPanel from './EditPanel/EditPanel';
import AddObjectPanel from './AddObjectPanel';
import { useGameContext } from "@/gameContext";
import { getFileBytes } from "@/index";

const loadSceneRandomSpheres = () => {
    console.log("Loading random spheres scene");
    wasm.load_scene_random_spheres();
}
const loadSceneCornellBox = () => {
    console.log("Loading Cornell Box scene");
    wasm.load_scene_cornell_box();
}
const loadSceneFantasyBook = async () => {
    console.log("Loading fantasy book scene");
    const glbBytes = await getFileBytes("../static/medieval_fantasy_book.glb");
    wasm.load_scene_fantasy_book(glbBytes);
}
const loadSceneMagicBridge = async () => {
    console.log("Loading magic bridge scene");
    const glbBytes = await getFileBytes("../static/magical_help.glb");
    wasm.load_scene_magic_bridge(glbBytes);
}
const loadSceneSimpleLight = () => {
    console.log("Loading simple light scene");
    wasm.load_scene_simple_light();
}
const loadSceneCornellBoxPlusPlus = async () => {
    console.log("Loading Cornell Box++ scene");
    const stlBytes = await getFileBytes("../static/angel.stl");
    wasm.load_scene_cornell_box_extra(stlBytes);
}

const loadSceneGandalfBust = async () => {
    console.log("Loading Gandalf bust scene");
    const glbBytes = await getFileBytes("../static/gandalf_bust.stl");
    wasm.load_scene_gandalf_bust(glbBytes);
}
const loadSceneRozaBust = async () => {
    console.log("Loading Roza bust scene");
    const glbBytes = await getFileBytes("../static/roza_bust.glb");
    wasm.load_scene_roza_bust(glbBytes);
}
const loadSceneDragon = async () => {
    console.log("Loading Dragon scene");
    const stlBytes = await getFileBytes("../static/dragon.stl");
    wasm.load_scene_dragon(stlBytes);
}
const loadSceneMirrorBox = async () => {
    console.log("Loading Mirror Box scene");
    const skullStlBytes = await getFileBytes("../static/skull.stl");
    const sculptureStlBytes = await getFileBytes("../static/abstract_sculpture.stl");
    wasm.load_scene_mirror_box(skullStlBytes, sculptureStlBytes);
}
const loadSceneSuzanneMonkey = async () => {
    console.log("Loading Suzanne Monkey scene");
    const suzanneStlBytes = await getFileBytes("../static/suzanne.stl");
    wasm.load_scene_suzanne_monkey(suzanneStlBytes);
}


setTimeout(() => {
    // loadSceneGandalfBust();
    // loadSceneRozaBust();
    // loadSceneDragon();
    // loadSceneMirrorBox();
    loadSceneSuzanneMonkey();
}, 1000)


const SceneControlPanel: React.FC = () => {

    const {
        selectedObjMatProps,
        gameStatus,
        followCamera,
        fov,
        focalDistance,
        dofStrength,
    } = useGameContext();

    // state to control which accordion items are open
    const [openAccordionItems, setOpenAccordionItems] = useState<string[]>([]);

    // --- handlers ---
    const handleLoadDefaultScene = (sceneValue: string) => {
        console.log(`Context: Requesting WASM to load default scene: ${sceneValue}`);
        switch (sceneValue) {
            case "Random Spheres":
                loadSceneRandomSpheres();
                break;
            case "Cornell Box":
                loadSceneCornellBox();
                break;
            case "Fantasy Book":
                loadSceneFantasyBook();
                break;
            case "Magic Bridge (incomplete)":
                loadSceneMagicBridge();
                break;
            case "Simple Light":
                loadSceneSimpleLight();
                break;
            case "Cornell Box++":
                loadSceneCornellBoxPlusPlus();
                break;
            case "Statue Bust":
                loadSceneRozaBust();
                break;
            case "Dragon":
                loadSceneDragon();
                break;
            case "Mirror Box":
                loadSceneMirrorBox();
                break;
            case "Suzanne Monkey":
                loadSceneSuzanneMonkey();
                break;
            // Add more cases for other scenes
            default:
                console.warn(`Unknown default scene value: ${sceneValue}`);
        }
    };

    const handleEnterRayTraceMode = () => {
        console.log("Context: Requesting WASM to enter ray trace mode");
        wasm.enter_ray_tracing_mode();
    };

    const handleStopRayTracing = () => {
        console.log("Context: Requesting WASM to stop ray tracing");
        wasm.stop_ray_tracing();
    };

    const handleFovChange = (value: number[]) => {
        const newFov = value[0];
        console.log(`Context: Setting FOV to ${newFov} via WASM`);

        const fov_radians = (newFov * Math.PI) / 180; // Convert degrees to radians
        wasm.set_fov(fov_radians); 
    };

    const handleToggleRealtimeLighting = (checked: boolean) => {
        console.log(`Context: Toggling real-time lighting to ${checked} via WASM`);
        if (checked) {
            wasm.exit_edit_mode();
        } else {
            wasm.enter_edit_mode();
        }
    };

    const handleFocalDistanceChange = (value: number[]) => {
        const newFocalDistance = value[0];
        console.log(`Context: Setting Focal Distance to ${newFocalDistance} via WASM`);
        wasm.set_focal_dist(newFocalDistance);
    };

    const handleDofStrengthChange = (value: number[]) => {
        const newDofStrength = value[0];
        console.log(`Context: Setting Depth of Field Strength to ${newDofStrength} via WASM`);
        // dof goes from 0 to 100, but defocus angle is in radians.
        // let max defocus angle be 0.05 radians.
        const radians = newDofStrength / 2000;
        wasm.set_defocus_angle(radians);
    };

    const inEditMode = gameStatus === 'Editing';
    const inRayTracingMode = gameStatus === 'RayTracing';

    const showAddObjectTrigger = gameStatus === 'Editing';
    const canEditSelectedObject = gameStatus === 'Editing' && selectedObjMatProps;

    useEffect(() => {
        // If an object is deselected (selectedObjMatProps becomes null)
        // while in edit mode, ensure the edit panel is closed.
        if (inEditMode && !selectedObjMatProps) {
            setOpenAccordionItems(prevItems =>
                prevItems.filter(item => item !== 'edit-selected-object-panel')
            );
        }
        
        // If not in edit mode, ensure all accordion items controlled here are closed.
        // The `hidden` prop on Accordion already hides it, but this keeps state consistent.
        if (!inEditMode) {
            setOpenAccordionItems([]);
        }
        // This effect primarily handles closing the edit panel on deselection
        // or when exiting edit mode. Opening panels is handled by user interaction
        // or explicitly in mode change handlers (like handleEnterEditMode).
    }, [selectedObjMatProps, inEditMode]); // Depend on inEditMode directly

    return (
        <Card className="w-full h-full overflow-y-auto rounded-none border-0">
            <CardHeader>
                <CardTitle>Scene Controls</CardTitle>
                <CardDescription>Manage and edit your 3D scene.</CardDescription>
            </CardHeader>
            <CardContent className="space-y-6 pb-20">

                {/* Load Default Scene Dropdown */}
                <div className="space-y-2 pt-2">
                    <div className="flex justify-between items-center">
                        <Label htmlFor="load-scene-select" className="text-sm font-medium">Load Default Scene</Label>
                    </div>
                    <Select
                        onValueChange={handleLoadDefaultScene}
                        disabled={!inEditMode}
                        defaultValue="Random Spheres"
                    >
                        <SelectTrigger id="load-scene-select" className="w-full">
                            <SelectValue placeholder="Select a scene" />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="Cornell Box++">Cornell Box++</SelectItem>
                            <SelectItem value="Dragon">Dragon</SelectItem>
                            <SelectItem value="Statue Bust">Statue Bust</SelectItem>
                            <SelectItem value="Fantasy Book">Fantasy Book</SelectItem>
                            <SelectItem value="Simple Light">Simple Light</SelectItem>
                            <SelectItem value="Random Spheres">Random Spheres</SelectItem>
                            <SelectItem value="Mirror Box">Mirror Box</SelectItem>
                            <SelectItem value="Suzanne Monkey">Suzanne Monkey</SelectItem>
                            {/* <SelectItem value="Cornell Box">Cornell Box</SelectItem>
                            <SelectItem value="Magic Bridge (incomplete)">Magic Bridge (incomplete)</SelectItem> */}
                        </SelectContent>
                    </Select>
                </div>

                {/* FOV Slider */}
                <div className="space-y-2 pt-2">
                    <div className="flex justify-between items-center">
                        <Label htmlFor="fov-slider" className="text-sm font-medium">Field of View</Label>
                        {/* Display FOV from context, provide a fallback if fov might be undefined initially */}
                        <span className="text-sm text-muted-foreground">{(fov ?? 90).toFixed(0)}°</span>
                    </div>
                    <Slider
                        id="fov-slider"
                        disabled={inRayTracingMode}
                        min={10}
                        max={140}
                        step={1}
                        // Use fov from context, provide a fallback for initial render if needed
                        value={[fov ?? 90]} 
                        onValueChange={handleFovChange}
                        className="w-full"
                    />
                </div>

                {/* Focal Distance Slider */}
                <div className="space-y-2 pt-2">
                    <div className="flex justify-between items-center">
                        <Label htmlFor="focal-distance-slider" className="text-sm font-medium">Focal Distance</Label>
                        <span className="text-sm text-muted-foreground">{(focalDistance ?? 10).toFixed(1)}</span>
                    </div>
                    <Slider
                        id="focal-distance-slider"
                        disabled={inRayTracingMode}
                        min={1}
                        max={50}
                        step={0.1}
                        value={[focalDistance ?? 10]}
                        onValueChange={handleFocalDistanceChange}
                        className="w-full"
                    />
                </div>

                {/* Depth of Field Strength Slider */}
                <div className="space-y-2 pt-2">
                    <div className="flex justify-between items-center">
                        <Label htmlFor="dof-strength-slider" className="text-sm font-medium">Depth of Field Strength</Label>
                        <span className="text-sm text-muted-foreground">{(dofStrength ?? 0).toFixed(0)}</span>
                    </div>
                    <Slider
                        id="dof-strength-slider"
                        disabled={inRayTracingMode}
                        // arbitrary min/max values
                        min={0}
                        max={100}
                        step={1}
                        value={[dofStrength ?? 0]}
                        onValueChange={handleDofStrengthChange}
                        className="w-full"
                    />
                </div>

                {/* Top Level Mode Buttons */}
                <div className="mb-4">

                    {/* Ray Trace / Stop Ray Tracing Button Slot */}
                    {inRayTracingMode ? (
                        <Button
                            onClick={handleStopRayTracing}
                            className="w-full whitespace-normal break-words h-10"
                        >
                            Stop Ray Tracing
                        </Button>
                    ) : (
                        <Tooltip>
                            <TooltipTrigger asChild>
                                    <div className="w-full"> {/* Wrapper for TooltipTrigger when button is disabled */}
                                    <Button
                                        onClick={handleEnterRayTraceMode}
                                        className="w-full whitespace-normal break-words h-10 bg-green-600 hover:bg-green-700"
                                    >
                                        Ray Trace
                                    </Button>
                                </div>
                            </TooltipTrigger>
                                <TooltipContent>
                                    <p>Generate a photorealistic still image.</p>
                                </TooltipContent>
                        </Tooltip>
                    )}
                </div>

                {/* Real-time Shadows & Lighting Toggle */}
                <div className="flex items-center justify-between space-x-2 p-2">
                    <Label htmlFor="realtime-lighting-toggle" className="text-sm font-medium">
                        Real-time Shadows & Lighting
                    </Label>
                    <Switch
                        id="realtime-lighting-toggle"
                        checked={!inEditMode} // Provide a fallback if undefined initially
                        onCheckedChange={handleToggleRealtimeLighting}
                        disabled={inRayTracingMode} // Disable if ray tracing
                    />
                </div>
                <p className="text-sm text-muted-foreground">
                    {!inEditMode ? "Scene editing unavailable when using real-time lighting." : ""}
                </p>

                <Accordion
                    type="multiple"
                    className="w-full"
                    hidden={!inEditMode} 
                    value={openAccordionItems}
                    onValueChange={setOpenAccordionItems}
                >
                    {/* Add Object Panel Accordion Item */}
                    <AccordionItem value="add-object-panel">
                        <AccordionTrigger disabled={!showAddObjectTrigger}>
                            Add New Object
                        </AccordionTrigger>
                        <AccordionContent>
                            {showAddObjectTrigger ? (
                                <AddObjectPanel/>
                            ) : (
                                <p className="text-sm text-muted-foreground p-4 text-center">
                                    Adding objects is only available in Edit Mode.
                                </p>
                            )}
                        </AccordionContent>
                    </AccordionItem>

                    {/* Edit Selected Object Panel Accordion Item */}
                    <AccordionItem value="edit-selected-object-panel">
                        <AccordionTrigger 
                            disabled={!canEditSelectedObject}
                            className={!canEditSelectedObject ? "text-muted-foreground/70 cursor-not-allowed" : ""}
                        >
                            Edit Selected Object
                        </AccordionTrigger>
                        <AccordionContent>
                            {canEditSelectedObject ? (
                                <EditPanel/>
                            ) : (
                                <p className="text-sm text-muted-foreground p-4 text-center">
                                    {gameStatus === 'Editing' ? "No object selected. Select an object in the scene to edit its properties." : "Object editing is only available in Edit Mode."}
                                </p>
                            )}
                        </AccordionContent>
                    </AccordionItem>
                </Accordion>

            </CardContent>
        </Card>
    );
};

export default SceneControlPanel;