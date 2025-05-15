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

import EditPanel from './EditPanel/EditPanel';
import AddObjectPanel from './AddObjectPanel';
import { useGameContext } from "@/gameContext";



const SceneControlPanel: React.FC = () => {

    const {
        selectedObjMatProps,
        gameStatus,
        followCamera,
        fov,
    } = useGameContext();

    // state to control which accordion items are open
    const [openAccordionItems, setOpenAccordionItems] = useState<string[]>([]);

    // --- handlers ---
    const handleEnterEditMode = () => {
        console.log("Context: Requesting WASM to enter edit mode");
        wasm.enter_edit_mode();
    };

    const handleExitEditMode = () => {
        console.log("Context: Requesting WASM to exit edit mode");
        wasm.exit_edit_mode();
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
                    {!inEditMode ? "Scene-editing unavailable when using real-time lighting." : ""}
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