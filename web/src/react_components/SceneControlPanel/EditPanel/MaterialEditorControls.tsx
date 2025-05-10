import * as wasm from "@wasm/wasm_graphics"

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

// Mock WASM function calls
const wasmSetMaterialColor = (color: string) => {
    console.log(`JS: Setting material color to ${color}`);
    const [r, g, b] = hexToFloatColor(color);
    console.log(`JS: Converted color to RGB: ${r}, ${g}, ${b}`);
    wasm.set_material_color(r, g, b);
}
const wasmSetMaterialType = (materialType: number) => {
    console.log(`JS: Setting material type to ${materialType}`);
    // wasm.set_material_type(materialType);
}
const wasmSetMaterialIOR = (ior: number) => {
    console.log(`JS: Setting material IOR to ${ior}`);
    // wasm.set_material_ior(ior);
}
const wasmSetMaterialRoughness = (roughness: number) => {
    console.log(`JS: Setting material roughness to ${roughness}`);
    // wasm.set_material_roughness(roughness);
}
const wasmSetMaterialBrightness = (brightness: number) => {
    console.log(`JS: Setting material brightness to ${brightness}`);
    // wasm.set_material_brightness(brightness);
}

const wasmGetSelectedObjectMaterial = (): wasm.MaterialProperties | null | undefined => {
    console.log("JS MaterialEditorControls: Querying WASM for selected object material");

    if (wasm.is_object_selected()) { // You'll need such a function in WASM
        return wasm.get_selected_material_properties();
    }
    return null;
};

// Utility functions
const floatColorToHex = (r: number, g: number, b: number): [string, number] => {
    // r,g,b given as floats between 0 and 1.
    // if any are above 1.0, this means the object is a light, so normalize color so max component
    // is 1.0, and return the brightness as the amount the max component was divided by.
    // If not light, return 1.0 as brightness.
    const maxComponent = Math.max(r, g, b);
    const brightness = maxComponent > 1.0 ? maxComponent : 1.0;
    r /= brightness;
    g /= brightness;
    b /= brightness;
    const hexColor = `#${Math.round(r * 255).toString(16).padStart(2, '0')}${Math.round(g * 255).toString(16).padStart(2, '0')}${Math.round(b * 255).toString(16).padStart(2, '0')}`;
    return [hexColor, brightness];
}

const hexToFloatColor = (hex: string): [number, number, number] => {
    // Convert hex color to float RGB values between 0 and 1
    const bigint = parseInt(hex.slice(1), 16);
    const r = ((bigint >> 16) & 255) / 255;
    const g = ((bigint >> 8) & 255) / 255;
    const b = (bigint & 255) / 255;
    return [r, g, b];
}

interface MaterialEditorControlsProps {
    selectionVersion: number;
}

const MaterialEditorControls: React.FC<MaterialEditorControlsProps> = ({ selectionVersion }) => {

    const [matIsEditable, setMatIsEditable] = useState<boolean>(false);

    const [color, setColor] = useState<string>("#FFFFFF");
    const [materialType, setMaterialType] = useState<number>(0);
    
    // Material-specific properties
    const [ior, setIor] = useState<number>(1.5); // For Glass
    const [roughness, setRoughness] = useState<number>(0.0); // For Metal
    const [brightness, setBrightness] = useState<number>(5.0); // For Light

    // useEffect to fetch material properties when selectionVersion changes
    useEffect(() => {
        console.log(`MaterialEditorControls: selectionVersion changed to ${selectionVersion}. Fetching material properties.`);
        if (selectionVersion > 0) {
            const materialProps = wasmGetSelectedObjectMaterial();
            if (materialProps) {
                let [hexColor, brightness] = floatColorToHex(materialProps.r, materialProps.g, materialProps.b);
                let materialType = materialProps.material_type;
                let extraProp = materialProps.extra_prop;
                setMatIsEditable(materialProps.mat_is_editable);
                setColor(hexColor);
                setMaterialType(materialType);

                // Set specific properties based on type, with defaults if not present in props
                setRoughness(materialType === 1 ? extraProp ?? 0.0 : 0.0);
                setIor(materialType === 2 ? extraProp ?? 1.5 : 1.5);
                setBrightness(materialType === 3 ? brightness ?? 5.0 : 5.0);
            } else {
                // No properties returned (e.g., no object selected or error), reset to defaults
                setMatIsEditable(false);
                setColor("#000000");
                setMaterialType(0);
                setIor(1.5);
                setRoughness(0.0);
                setBrightness(5.0);
            }
        } else {
            // selectionVersion indicates no selection or initial state, reset to defaults
            setMatIsEditable(false);
            setColor("#000000");
            setMaterialType(0);
            setIor(1.5);
            setRoughness(0.0);
            setBrightness(5.0);
        }
    }, [selectionVersion]); // Dependency array: re-run effect when selectionVersion changes

    // Handlers for property changes
    const handleColorChange = (newColor: string) => {
        setColor(newColor);
        wasmSetMaterialColor(newColor);
    };

    const handleMaterialTypeChange = (newType: number) => {
        setMaterialType(newType);
        wasmSetMaterialType(newType);
    };

    const handleIorChange = (newIor: number) => {
        if (materialType !== 2 || !matIsEditable) {
            console.error("IOR change attempted on non-glass material type.");
        }
        const clampedIor = Math.max(0.1, Math.min(5.0, newIor));
        setIor(clampedIor);
        wasmSetMaterialIOR(clampedIor);
    };

    const handleRoughnessChange = (newRoughness: number) => {
        if (materialType !== 1 || !matIsEditable) {
            console.error("Roughness change attempted on non-metal material type.");
        }
        const clampedRoughness = Math.max(0.0, Math.min(1.0, newRoughness));
        setRoughness(clampedRoughness);
        wasmSetMaterialRoughness(clampedRoughness);
    };

    const handleBrightnessChange = (newBrightness: number) => {
        if (materialType !== 3 || !matIsEditable) {
            console.error("Brightness change attempted on non-light material type.");
        }
        const clampedBrightness = Math.max(0.1, Math.min(100.0, newBrightness));
        setBrightness(clampedBrightness);
        wasmSetMaterialBrightness(clampedBrightness);
    };


    return (
        <AccordionItem value="materialEditor">
            <AccordionTrigger disabled={!matIsEditable} className={!matIsEditable ? "cursor-not-allowed text-muted-foreground/70" : ""}>
                Material Editor {!matIsEditable && <span className="text-xs ml-2">(Not Editable)</span>}
            </AccordionTrigger>
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
                            disabled={!matIsEditable}
                        />
                    </div>

                    {/* Material Type Dropdown */}
                    <div className="flex-1 space-y-1"> {/* Use flex-1 to take up available space */}
                        <Label htmlFor="material-type" className="text-sm font-medium">Material Type</Label>
                        <Select 
                            value={materialType.toString()} 
                            onValueChange={(value: string) => {handleMaterialTypeChange(parseInt(value))}}
                            disabled={!matIsEditable}
                        >
                            <SelectTrigger id="material-type" className="h-10"> {/* Match height of color input */}
                                <SelectValue placeholder="Select material type" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="0">Diffuse</SelectItem>
                                <SelectItem value="1">Metal</SelectItem>
                                <SelectItem value="2">Glass</SelectItem>
                                <SelectItem value="3">Light</SelectItem>
                            </SelectContent>
                        </Select>
                    </div>
                </div>

                {/* Conditional Inputs based on Material Type */}
                {materialType === 1 && (
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
                            disabled={!matIsEditable || materialType !== 1}
                        />
                    </div>
                )}

                {materialType === 2 && (
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
                            disabled={!matIsEditable || materialType !== 2}
                        />
                    </div>
                )}

                {materialType === 3 && (
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
                            disabled={!matIsEditable || materialType !== 3}
                        />
                    </div>
                )}
            </AccordionContent>
        </AccordionItem>
    );
};

export default MaterialEditorControls;