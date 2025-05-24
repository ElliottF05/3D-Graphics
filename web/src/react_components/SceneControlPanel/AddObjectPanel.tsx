import * as wasm from "@wasm/wasm_graphics";

import React, { useState, useRef } from 'react';
import {
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
} from "@/components/ui/accordion";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";
import { getGlbBytes } from "@/index";

type ObjectType = 'Sphere' | 'Box' | 'Custom';

// Mock WASM function calls for adding objects
const wasmAddCustomObject = async (file: File) => {
    console.log(`WASM: Add Custom Object from file ${file.name}`);
    const glbBuffer = await file.arrayBuffer();
    const glbBytes = new Uint8Array(glbBuffer);
    if (glbBytes) {
        wasm.add_custom_object(glbBytes);
    } else {
        console.error("Failed to load GLB bytes for custom object, glbBytes is null or undefined.");
    }
};

interface AddObjectPanelProps {
}

const AddObjectPanel: React.FC<AddObjectPanelProps> = () => {
    const [objectType, setObjectType] = useState<ObjectType>('Sphere');
    // Sphere state
    const [radius, setRadius] = useState<number>(1.0);
    // Box state
    const [boxLength, setBoxLength] = useState<number>(1.0);
    const [boxWidth, setBoxWidth] = useState<number>(1.0);
    const [boxHeight, setBoxHeight] = useState<number>(1.0);
    // Custom object state
    const [customFile, setCustomFile] = useState<File | null>(null);
    const fileInputRef = useRef<HTMLInputElement>(null);

    const handleAddObjectClick = () => {
        switch (objectType) {
            case 'Sphere':
                wasm.add_sphere(radius);
                break;
            case 'Box':
                wasm.add_box(boxLength, boxWidth, boxHeight);
                break;
            case 'Custom':
                if (customFile) {
                    wasmAddCustomObject(customFile);
                } else {
                    alert("Please select a .glb file for the custom object.");
                }
                break;
        }
    };

    const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        if (event.target.files && event.target.files[0]) {
            if (event.target.files[0].name.endsWith('.glb')) {
                setCustomFile(event.target.files[0]);
            } else {
                alert("Please select a .glb file.");
                if(fileInputRef.current) {
                    fileInputRef.current.value = ""; // Reset file input
                }
                setCustomFile(null);
            }
        } else {
            setCustomFile(null);
        }
    };

    return (
    <div className="space-y-4 pt-2">
        {/* Object Type Dropdown */}
        <div className="space-y-1">
            <Label htmlFor="object-type-select" className="text-sm font-medium">Object Type</Label>
            <Select value={objectType} onValueChange={(value: ObjectType) => setObjectType(value)}>
                <SelectTrigger id="object-type-select">
                    <SelectValue placeholder="Select object type" />
                </SelectTrigger>
                <SelectContent>
                    <SelectItem value="Sphere">Sphere</SelectItem>
                    <SelectItem value="Box">Box</SelectItem>
                    <SelectItem value="Custom">Custom (.glb) - experimental!</SelectItem>
                </SelectContent>
            </Select>
        </div>

        {/* Conditional Inputs */}
        {objectType === 'Sphere' && (
            <div className="space-y-1 animate-fadeIn">
                <Label htmlFor="sphere-radius" className="text-xs text-muted-foreground">Radius</Label>
                <Input
                    id="sphere-radius"
                    type="number"
                    value={radius}
                    onChange={(e) => setRadius(Math.max(0.1, parseFloat(e.target.value) || 0.1))}
                    step="0.1"
                    min="0.1"
                    className="w-full h-8 text-xs"
                />
            </div>
        )}

        {objectType === 'Box' && (
            <div className="space-y-3 animate-fadeIn">
                <div>
                    <Label htmlFor="box-length" className="text-xs text-muted-foreground">Length</Label>
                    <Input id="box-length" type="number" value={boxLength}
                        onChange={(e) => setBoxLength(Math.max(0.1, parseFloat(e.target.value) || 0.1))}
                        step="0.1" min="0.1" className="w-full h-8 text-xs" />
                </div>
                <div>
                    <Label htmlFor="box-width" className="text-xs text-muted-foreground">Width</Label>
                    <Input id="box-width" type="number" value={boxWidth}
                        onChange={(e) => setBoxWidth(Math.max(0.1, parseFloat(e.target.value) || 0.1))}
                        step="0.1" min="0.1" className="w-full h-8 text-xs" />
                </div>
                <div>
                    <Label htmlFor="box-height" className="text-xs text-muted-foreground">Height</Label>
                    <Input id="box-height" type="number" value={boxHeight}
                        onChange={(e) => setBoxHeight(Math.max(0.1, parseFloat(e.target.value) || 0.1))}
                        step="0.1" min="0.1" className="w-full h-8 text-xs" />
                </div>
            </div>
        )}

        {objectType === 'Custom' && (
            <div className="space-y-1 animate-fadeIn">
                <Label htmlFor="custom-file-upload" className="text-xs text-muted-foreground">Upload .glb File</Label>
                <Input
                    id="custom-file-upload"
                    type="file"
                    accept=".glb"
                    ref={fileInputRef}
                    onChange={handleFileChange}
                    className="w-full h-10 text-xs file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-semibold file:bg-primary file:text-primary-foreground hover:file:bg-primary/90"
                />
                {customFile && <p className="text-xs text-muted-foreground mt-1">Selected: {customFile.name}</p>}
            </div>
        )}

        <Button className="w-full mt-4" onClick={handleAddObjectClick}>
            Add Object to Scene
        </Button>
    </div>
    );
};

export default AddObjectPanel;