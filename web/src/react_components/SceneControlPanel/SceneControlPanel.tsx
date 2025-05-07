import React, { useState } from 'react';
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
// Input, Label, Switch might be used for other top-level controls or other accordion items later
// import { Input } from "@/components/ui/input";
// import { Label } from "@/components/ui/label";
// import { Switch } from "@/components/ui/switch";
import {
    Accordion,
    AccordionContent,
    // AccordionContent, // Will be used by EditPanel or other items
    AccordionItem,
    AccordionTrigger,
} from "@/components/ui/accordion";
// Select components might be used for an "Add Object" panel later
// import {
//     Select,
//     SelectContent,
//     SelectItem,
//     SelectTrigger,
//     SelectValue,
// } from "@/components/ui/select";

import EditPanel from './EditPanel/EditPanel'; // Import the EditPanel


const SceneControlPanel: React.FC = () => {

    return (
        <Card className="w-full h-full overflow-y-auto rounded-none border-0">
            <CardHeader>
                <CardTitle>Scene Controls</CardTitle>
                <CardDescription>Manage and edit your 3D scene.</CardDescription>
            </CardHeader>
            <CardContent className="space-y-6 pb-20"> {/* Added padding-bottom for scroll */}
                <Accordion type="multiple" className="w-full" defaultValue={['edit-panel']}>
                    {/* Edit Panel Section */}
                    {/* The EditPanel itself contains AccordionItems, so we don't wrap EditPanel in AccordionContent here.
                        Instead, EditPanel will render its own AccordionItem structure.
                        If EditPanel was just content, we'd use AccordionContent.
                        Let's adjust EditPanel to be a direct child or make it return its own AccordionItem.
                        For now, let's assume EditPanel is designed to be a top-level section.
                    */}
                    <AccordionItem value="edit-panel-wrapper">
                        <AccordionTrigger>Edit Selected Object</AccordionTrigger>
                        {/* The EditPanel component already contains its own Accordion and AccordionItems.
                            So, we place it directly inside an AccordionContent of this higher-level accordion.
                        */}
                        <AccordionContent>
                            <EditPanel />
                        </AccordionContent>
                    </AccordionItem>
                    

                    {/* Placeholder for Add Object Section */}
                    {/* 
                    <AccordionItem value="add-object">
                        <AccordionTrigger>Add Object</AccordionTrigger>
                        <AccordionContent className="space-y-4 pt-2">
                            <p>Add object controls will go here...</p>
                            <div>
                                <Label htmlFor="object-type">Type</Label>
                                <Select onValueChange={(value) => console.log("New object type:", value)}>
                                    <SelectTrigger id="object-type"><SelectValue placeholder="Select type" /></SelectTrigger>
                                    <SelectContent>
                                        <SelectItem value="cube">Cube</SelectItem>
                                        <SelectItem value="sphere">Sphere</SelectItem>
                                    </SelectContent>
                                </Select>
                            </div>
                            <Button className="w-full" onClick={() => console.log("Add object clicked")}>Add to Scene</Button>
                        </AccordionContent>
                    </AccordionItem>
                    */}

                    {/* You can add more AccordionItem components here for other functionalities */}
                    
                </Accordion>
            </CardContent>
        </Card>
    );
};

export default SceneControlPanel;