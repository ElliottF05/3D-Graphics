import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogTrigger } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { HelpCircle } from "lucide-react";

const InstructionsDialog = () => (
    <Dialog>
        <DialogTrigger asChild>
            <Button variant="ghost" size="icon" className="absolute top-2 right-2" aria-label="Show instructions">
                <HelpCircle className="w-5 h-5" />
            </Button>
        </DialogTrigger>
        <DialogContent>
            <DialogHeader>
                <DialogTitle>Instructions & Controls</DialogTitle>
                <DialogDescription>
                    <div className="space-y-3 mt-2">
                        <div>
                            <strong className="text-base text-primary">Explore the scenes!</strong>
                            <ul className="list-disc pl-5 space-y-1 text-sm mt-1">
                                <li>Click on the canvas to enter the scene.</li>
                                <li>
                                    <span className="font-medium">Move:</span> <kbd className="px-1">W</kbd> <kbd className="px-1">A</kbd> <kbd className="px-1">S</kbd> <kbd className="px-1">D</kbd>
                                </li>
                                <li>
                                    <span className="font-medium">Look around:</span> Move your mouse
                                </li>
                                <li>
                                    <span className="font-medium">Go up/down:</span> <kbd className="px-1">Shift</kbd> (down), <kbd className="px-1">Space</kbd> (up)
                                </li>
                                <li>
                                    <span className="font-medium">Select objects:</span> Click on them in the scene
                                </li>
                                <li>
                                    <span className="font-medium">Edit properties:</span> Use the Edit Panel after selecting an object
                                </li>
                            </ul>
                        </div>
                        <div>
                            <strong className="text-base text-primary">Controls & Features</strong>
                            <ul className="list-disc pl-5 space-y-1 text-sm mt-1">
                                <li>Use the dropdown to load a default scene.</li>
                                <li>Adjust camera FOV, focal distance, and depth of field with the sliders.</li>
                                <li>Switch between real-time and ray-tracing modes using the toggle and button.</li>
                            </ul>
                        </div>
                        <div>
                            <strong className="text-base text-green-600">Ray Tracing</strong>
                            <div className="text-sm mt-1">
                                For the most realistic lighting and shadows, <span className="font-semibold text-primary">enable Ray Tracing mode</span>! <br />
                                <span className="italic">Explore scenes and see them rendered with physically accurate light for stunning results.</span>
                            </div>
                        </div>
                        <div className="text-xs text-muted-foreground pt-2">
                            For more help, see the project README or documentation.
                        </div>
                    </div>
                </DialogDescription>
            </DialogHeader>
        </DialogContent>
    </Dialog>
);

export default InstructionsDialog;