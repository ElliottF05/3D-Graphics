import React, { useState } from 'react';
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
// In a real app, these would call your actual WASM functions.
// You might also have WASM call JS functions to update 'isObjectSelectedFromWasm'
let isObjectSelectedInWasm = false; // Mock global state for WASM selection

const wasmDeselectCurrentObject = () => {
    console.log("JS: Telling WASM to deselect current object");
    isObjectSelectedInWasm = false;
    // Potentially, WASM could then call a JS callback to confirm/update React state
};
// --- End of Mock WASM Interaction ---


const SceneControlPanel: React.FC = () => {
    const [isObjectSelected, setIsObjectSelected] = useState<boolean>(false);
    const [activeMainAccordionItems, setActiveMainAccordionItems] = useState<string[]>(['add-object-panel']);
    const [editPanelOpenSubSections, setEditPanelOpenSubSections] = useState<string[]>(['transform']);
    const [editPanelAccordionKey, setEditPanelAccordionKey] = useState<string>('editPanelKey-initial');

    const handleObjectAddedFromPanel = () => {
        // When an object is added from AddObjectPanel:
        setActiveMainAccordionItems(['edit-panel-wrapper']);
        setEditPanelOpenSubSections(['transform', 'materialEditor']);
        setEditPanelAccordionKey(`editPanelKey-${Date.now()}`);
    };

    const handleActiveMainAccordionChange = (newActiveItems: string[]) => {
        const editPanelWasOpen = activeMainAccordionItems.includes('edit-panel-wrapper');
        const editPanelIsNowOpen = newActiveItems.includes('edit-panel-wrapper');

        if (editPanelWasOpen && !editPanelIsNowOpen && isObjectSelected) {
            // Edit panel was closed by the user
            wasmDeselectCurrentObject();
            setIsObjectSelected(false);
            console.log("Edit panel closed, object deselected.");
        }
        
        // If trying to open edit panel but no object is selected, prevent it.
        // (The disabled prop on AccordionItem should also handle this, but this is an extra check)
        if (!editPanelWasOpen && editPanelIsNowOpen && !isObjectSelected) {
            return; // Don't update state to open it
        }

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