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


// --- Mock WASM Interaction ---
let isObjectSelectedInWasm = false; 

const wasmIsAnythingSelected = (): boolean => {
    console.log("JS: Checking WASM if object is selected");
    return isObjectSelectedInWasm;
};

const wasmSelectObjectById = (id: string) => { 
    console.log(`JS: Telling WASM to select object ${id}`);
    isObjectSelectedInWasm = true;
};

const wasmDeselectCurrentObject = () => {
    console.log("JS: Telling WASM to deselect current object");
    isObjectSelectedInWasm = false;
};

const wasmConfirmObjectEditsWasm = () => { // Renamed to avoid conflict with handler
    console.log("JS: Telling WASM edits are confirmed for selected object");
    // Here WASM might finalize or apply pending changes
    // For this UI flow, we'll deselect after confirming.
    isObjectSelectedInWasm = false; 
};
// --- End Mock WASM Interaction ---


const SceneControlPanel: React.FC = () => {
    const [isObjectSelected, setIsObjectSelected] = useState<boolean>(false);
    const [activeMainAccordionItems, setActiveMainAccordionItems] = useState<string[]>(['add-object-panel']);
    const [editPanelOpenSubSections, setEditPanelOpenSubSections] = useState<string[]>(['transform']);
    const [editPanelAccordionKey, setEditPanelAccordionKey] = useState<string>('editPanelKey-initial');

    // Effect to register the React state setter to the global bridge
    useEffect(() => {
        (window as any).wasmBridge.jsSetIsObjectSelected = (isSelected: boolean) => {
            console.log(`JS: updateIsObjectSelectedFromWasm called by WASM with: ${isSelected}`);
            setIsObjectSelected(isSelected);
        };

        // Cleanup function when the component unmounts
        return () => {
            (window as any).wasmBridge.jsSetIsObjectSelected = (isSelected: boolean) => {
                console.warn("WASM tried to update selection, but React component (SceneControlPanel) was unmounted.");
            };
        };
    }, [setIsObjectSelected]); // Dependency: re-run if setIsObjectSelected changes (though unlikely for setters)


    const handleObjectAddedFromPanel = () => {
        setIsObjectSelected(true); 
        setActiveMainAccordionItems(['edit-panel-wrapper']);
        setEditPanelOpenSubSections(['transform', 'materialEditor']);
    };

    const handleConfirmEditFromPanel = () => {
        wasmConfirmObjectEditsWasm();
        setIsObjectSelected(false);
        setActiveMainAccordionItems(prev => prev.filter(item => item !== 'edit-panel-wrapper')); // Close the EditPanel
        console.log("Edits confirmed, panel closed, object deselected.");
    };

    const handleDeleteObjectFromPanel = () => {
        // WASM's delete function should handle its internal selection state.
        // We just update React's view of selection.
        setIsObjectSelected(false);
        setActiveMainAccordionItems(prev => prev.filter(item => item !== 'edit-panel-wrapper')); // Close the EditPanel
        console.log("Object deleted, panel closed, object deselected in UI.");
    };

    const handleActiveMainAccordionChange = (newActiveItems: string[]) => {
        setActiveMainAccordionItems(newActiveItems);
    };

    const canOpenEditPanel = isObjectSelected;

    return (
        <Card className="w-full h-full overflow-y-auto rounded-none border-0">
            <CardHeader>
                <CardTitle>Scene Controls</CardTitle>
                <CardDescription>Manage and edit your 3D scene.</CardDescription>
            </CardHeader>
            <CardContent className="space-y-6 pb-20">
                <Accordion 
                    type="multiple" 
                    className="w-full"
                    value={activeMainAccordionItems}
                    onValueChange={handleActiveMainAccordionChange}
                >
                    {/* Add Object Panel Section */}
                    <AddObjectPanel onObjectAdded={handleObjectAddedFromPanel} />
                    
                    {/* Edit Panel Section Wrapper */}
                    <AccordionItem value="edit-panel-wrapper">
                        <AccordionTrigger
                                className={!canOpenEditPanel ? "text-muted-foreground/50 cursor-not-allowed" : ""}
                        >
                            Edit Selected Object
                        </AccordionTrigger>
                        <AccordionContent>
                            {canOpenEditPanel ? (
                                <EditPanel 
                                    initialOpenSections={editPanelOpenSubSections}
                                    accordionKey={editPanelAccordionKey}
                                    onConfirmEdit={handleConfirmEditFromPanel}
                                    onDeleteObject={handleDeleteObjectFromPanel}
                                />
                            ) : (
                                <p className="text-sm text-muted-foreground p-4 text-center">
                                    No object selected. Add an object or select one in the scene to edit.
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