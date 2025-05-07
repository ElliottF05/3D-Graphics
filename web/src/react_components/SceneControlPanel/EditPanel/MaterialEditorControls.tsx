import React, { useState, useEffect } from 'react';
import {
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
} from "@/components/ui/accordion";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";

type MaterialType = 'Diffuse' | 'Glass' | 'Metal' | 'Light';

// Mock WASM function calls
const wasmSetMaterialColor = (color: string) => {
    console.log(`WASM: Set material color to ${color}`);
};
const wasmSetMaterialType = (type: MaterialType) => {
    console.log(`WASM: Set material type to ${type}`);
};
const wasmSetMaterialIOR = (ior: number) => {
    console.log(`WASM: Set material IOR to ${ior}`);
};
const wasmSetMaterialRoughness = (roughness: number) => {
    console.log(`WASM: Set material roughness to ${roughness}`);
};
const wasmSetMaterialBrightness = (brightness: number) => {
    console.log(`WASM: Set material brightness to ${brightness}`);
};


interface MaterialEditorControlsProps { }

const MaterialEditorControls: React.FC<MaterialEditorControlsProps> = () => {
    const [color, setColor] = useState<string>("#FFFFFF");
    const [materialType, setMaterialType] = useState<MaterialType>('Diffuse');
    
    // Material-specific properties
    const [ior, setIor] = useState<number>(1.5); // For Glass
    const [roughness, setRoughness] = useState<number>(0.0); // For Metal
    const [brightness, setBrightness] = useState<number>(5.0); // For Light

    // Handlers for property changes
    const handleColorChange = (newColor: string) => {
        setColor(newColor);
        wasmSetMaterialColor(newColor);
    };

    const handleMaterialTypeChange = (newType: MaterialType) => {
        setMaterialType(newType);
        wasmSetMaterialType(newType);
        // Reset/update specific params when type changes if necessary
        switch (newType) {
            case 'Glass':
                wasmSetMaterialIOR(ior);
                break;
            case 'Metal':
                wasmSetMaterialRoughness(roughness);
                break;
            case 'Light':
                wasmSetMaterialBrightness(brightness);
                break;
        }
    };

    const handleIorChange = (newIor: number) => {
        const clampedIor = Math.max(0.1, Math.min(5.0, newIor));
        setIor(clampedIor);
        if (materialType === 'Glass') {
            wasmSetMaterialIOR(clampedIor);
        }
    };

    const handleRoughnessChange = (newRoughness: number) => {
        const clampedRoughness = Math.max(0.0, Math.min(1.0, newRoughness));
        setRoughness(clampedRoughness);
        if (materialType === 'Metal') {
            wasmSetMaterialRoughness(clampedRoughness);
        }
    };

    const handleBrightnessChange = (newBrightness: number) => {
        const clampedBrightness = Math.max(0.1, Math.min(100.0, newBrightness));
        setBrightness(clampedBrightness);
        if (materialType === 'Light') {
            wasmSetMaterialBrightness(clampedBrightness);
        }
    };

    useEffect(() => {
        switch (materialType) {
            case 'Glass':
                wasmSetMaterialIOR(ior);
                break;
            case 'Metal':
                wasmSetMaterialRoughness(roughness);
                break;
            case 'Light':
                wasmSetMaterialBrightness(brightness);
                break;
        }
    }, []);


    return (
        <AccordionItem value="materialEditor">
            <AccordionTrigger>Material Editor</AccordionTrigger>
            <AccordionContent className="space-y-4 pt-2">
                {/* Color Picker and Material Type Dropdown on the same row */}
                <div className="flex space-x-4">
                    {/* Color Picker */}
                    <div className="flex-1 space-y-1"> {/* Use flex-1 to take up available space */}
                        <Label htmlFor="material-color" className="text-sm font-medium">Color</Label>
                        <Input
                            id="material-color"
                            type="color"
                            value={color}
                            onChange={(e) => handleColorChange(e.target.value)}
                            className="w-full h-10 p-1"
                        />
                    </div>

                    {/* Material Type Dropdown */}
                    <div className="flex-1 space-y-1"> {/* Use flex-1 to take up available space */}
                        <Label htmlFor="material-type" className="text-sm font-medium">Material Type</Label>
                        <Select value={materialType} onValueChange={(value: MaterialType) => handleMaterialTypeChange(value)}>
                            <SelectTrigger id="material-type" className="h-10"> {/* Match height of color input */}
                                <SelectValue placeholder="Select material type" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="Diffuse">Diffuse</SelectItem>
                                <SelectItem value="Metal">Metal</SelectItem>
                                <SelectItem value="Glass">Glass</SelectItem>
                                <SelectItem value="Light">Light</SelectItem>
                            </SelectContent>
                        </Select>
                    </div>
                </div>

                {/* Conditional Inputs based on Material Type */}
                {materialType === 'Glass' && (
                    <div className="space-y-1">
                        <Label htmlFor="material-ior" className="text-xs text-muted-foreground">Index of Refraction (IOR)</Label>
                        <Input
                            id="material-ior"
                            type="number"
                            value={ior}
                            onChange={(e) => handleIorChange(parseFloat(e.target.value))}
                            step="0.01"
                            min="0.1"
                            max="5.0"
                            className="w-full h-8 text-xs"
                        />
                    </div>
                )}

                {materialType === 'Metal' && (
                    <div className="space-y-1">
                        <Label htmlFor="material-roughness" className="text-xs text-muted-foreground">Roughness</Label>
                        <Input
                            id="material-roughness"
                            type="number"
                            value={roughness}
                            onChange={(e) => handleRoughnessChange(parseFloat(e.target.value))}
                            step="0.01"
                            min="0.0"
                            max="1.0"
                            className="w-full h-8 text-xs"
                        />
                    </div>
                )}

                {materialType === 'Light' && (
                    <div className="space-y-1">
                        <Label htmlFor="material-brightness" className="text-xs text-muted-foreground">Brightness</Label>
                        <Input
                            id="material-brightness"
                            type="number"
                            value={brightness}
                            onChange={(e) => handleBrightnessChange(parseFloat(e.target.value))}
                            step="0.1"
                            min="0.1"
                            max="100.0"
                            className="w-full h-8 text-xs"
                        />
                    </div>
                )}
            </AccordionContent>
        </AccordionItem>
    );
};

export default MaterialEditorControls;