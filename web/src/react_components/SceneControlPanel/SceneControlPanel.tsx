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

const mockTogglePause = () => console.log("Toggle Pause/Unpause");

const SceneControlPanel: React.FC = () => {
    // State to control which main accordion items are open
    const [activeMainAccordionItems, setActiveMainAccordionItems] = useState<string[]>(['add-object-panel']);
    // State to control which sub-sections of EditPanel should be open
    const [editPanelOpenSubSections, setEditPanelOpenSubSections] = useState<string[]>(['transform']);
    // State for forcing EditPanel's accordion to re-evaluate defaultValue
    const [editPanelAccordionKey, setEditPanelAccordionKey] = useState<string>('editPanelKey-initial');

    const handleObjectAddedFromPanel = () => {
        // When an object is added from AddObjectPanel:
        // 1. Set the EditPanel's wrapper to be the active main accordion item.
        setActiveMainAccordionItems(['edit-panel-wrapper']);
        // 2. Specify that both transform and material sections in EditPanel should be open.
        setEditPanelOpenSubSections(['transform', 'materialEditor']);
        // 3. Change the key for EditPanel's internal accordion to ensure it re-renders with new defaults.
        setEditPanelAccordionKey(`editPanelKey-${Date.now()}`);
    };

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
                    value={activeMainAccordionItems} // Controlled component
                    onValueChange={setActiveMainAccordionItems} // Allow user to open/close sections
                >
                    {/* Add Object Panel Section */}
                    <AddObjectPanel onObjectAdded={handleObjectAddedFromPanel} />
                    
                    {/* Edit Panel Section Wrapper */}
                    <AccordionItem value="edit-panel-wrapper">
                        <AccordionTrigger>Edit Selected Object</AccordionTrigger>
                        <AccordionContent>
                            <EditPanel 
                                initialOpenSections={editPanelOpenSubSections}
                                accordionKey={editPanelAccordionKey}
                            />
                        </AccordionContent>
                    </AccordionItem>
                </Accordion>
            </CardContent>
        </Card>
    );
};

export default SceneControlPanel;