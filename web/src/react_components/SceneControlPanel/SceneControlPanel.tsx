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

import EditPanel from './EditPanel/EditPanel';
import AddObjectPanel from './AddObjectPanel';
import { useGameContext } from "@/gameContext";

// --- Mock WASM Interaction ---

// --- End Mock WASM Interaction ---


const SceneControlPanel: React.FC = () => {

    const {
        selectedObjMatProps,
        gameStatus,
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
    };

    const inEditMode = gameStatus === 'Editing';
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

                {/* Top Level Mode Buttons */}
                <div className="flex mb-4 gap-2">
                    <Button
                        onClick={handleEnterEditMode}
                        disabled={gameStatus === 'Editing'}
                        hidden={gameStatus === 'Editing'}
                        className="flex-1 min-w-0 whitespace-normal break-words h-14"
                    >
                        Enter Edit Mode
                    </Button>

                    <Button
                        onClick={handleExitEditMode}
                        disabled={gameStatus !== 'Editing'}
                        hidden={gameStatus !== 'Editing'}
                        className="flex-1 min-w-0 whitespace-normal break-words h-14"
                    >
                        Exit Edit Mode
                    </Button>

                    <Button
                        onClick={handleEnterRayTraceMode}
                        disabled={gameStatus === 'RayTracing'}
                        className="flex-1 min-w-0 whitespace-normal break-words h-14"
                    >
                        Ray Trace
                    </Button>
                </div>

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