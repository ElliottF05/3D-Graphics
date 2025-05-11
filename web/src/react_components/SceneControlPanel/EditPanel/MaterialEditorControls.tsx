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
import { useGameContext } from "@/gameContext";

// Utility functions
const floatColorToHex = (r: number, g: number, b: number): [string, number] => {
    const maxComponent = Math.max(r, g, b);
    let brightness = 1.0; // Default brightness
    let outR = r;
    let outG = g;
    let outB = b;

    if (maxComponent > 1.0) {
        brightness = maxComponent; // This is the "emissive strength" or how much it was over 1.0
        outR /= brightness; // Normalize color components to be within [0, 1]
        outG /= brightness;
        outB /= brightness;
    }
    
    const toHex = (c: number) => Math.round(Math.min(1.0, Math.max(0.0, c)) * 255).toString(16).padStart(2, '0');
    const hexColor = `#${toHex(outR)}${toHex(outG)}${toHex(outB)}`;
    
    // If the original r,g,b were all <= 1.0, brightness here will be 1.0.
    // If any component was > 1.0, brightness will be that maxComponent value.
    // This 'brightness' is more like an emission factor if the material is a light.
    // The actual brightness state for the Light material type might be separate.
    return [hexColor, brightness]; 
};

const hexToFloatColor = (hex: string): [number, number, number] => {
    // Convert hex color to float RGB values between 0 and 1
    const bigint = parseInt(hex.slice(1), 16);
    const r = ((bigint >> 16) & 255) / 255;
    const g = ((bigint >> 8) & 255) / 255;
    const b = (bigint & 255) / 255;
    return [r, g, b];
}

// WASM interaction functions (these will call actual WASM bindings)
const wasmSetMaterialColor = (hexColor: string) => {
    const [r, g, b] = hexToFloatColor(hexColor);
    console.log(`JS: Setting material color to RGB: ${r}, ${g}, ${b}`);
    // wasm.set_material_color(r, g, b); // Assumes wasm.set_material_color exists
};
const wasmSetMaterialType = (type: number) => {
    console.log(`JS: Setting material type to ${type}`);
    // wasm.set_material_type(type); // Assumes wasm.set_material_type exists
};
const wasmSetMaterialIOR = (ior: number) => {
    console.log(`JS: Setting material IOR to ${ior}`);
    // wasm.set_material_ior(ior); // Assumes wasm.set_material_ior exists
};
const wasmSetMaterialRoughness = (roughness: number) => {
    console.log(`JS: Setting material roughness to ${roughness}`);
    // wasm.set_material_roughness(roughness); // Assumes wasm.set_material_roughness exists
};
const wasmSetMaterialBrightness = (brightness: number) => {
    console.log(`JS: Setting material brightness to ${brightness}`);
    // wasm.set_material_brightness(brightness); // Assumes wasm.set_material_brightness exists
};


interface MaterialEditorControlsProps {
    disabled?: boolean; // to disable from the parent if needed
}

const MaterialEditorControls: React.FC<MaterialEditorControlsProps> = ({ disabled }) => {
    const {
        selectedObjMatProps,
        gameStatus,
        followCamera,
    } = useGameContext();

    // local state for UI display, derived from context's selectedObjMatProps
    const [displayColor, setDisplayColor] = useState<string>("#FFFFFF");
    const [displayMaterialType, setDisplayMaterialType] = useState<number>(0); // 0: Diffuse, 1: Metal, 2: Glass, 3: Light
    const [displayIor, setDisplayIor] = useState<number>(1.5); // for Glass
    const [displayRoughness, setDisplayRoughness] = useState<number>(0.0); // for Metal
    const [displayBrightness, setDisplayBrightness] = useState<number>(5.0); // for Light material
    
    const isActuallyEditable = selectedObjMatProps?.mat_is_editable ?? false;
    const overallDisabled = disabled || gameStatus !== 'Editing' || !selectedObjMatProps;

    useEffect(() => {
        if (selectedObjMatProps) {
            const [hexColor, lightBrightnessFactor] = floatColorToHex(selectedObjMatProps.r, selectedObjMatProps.g, selectedObjMatProps.b);
            setDisplayColor(hexColor);
            setDisplayMaterialType(selectedObjMatProps.material_type);

            // Update specific properties based on material type from WASM
            switch (selectedObjMatProps.material_type) {
                case 1: // Metal
                    setDisplayRoughness(selectedObjMatProps.extra_prop);
                    break;
                case 2: // Glass
                    setDisplayIor(selectedObjMatProps.extra_prop);
                    break;
                case 3: // Light
                    // The `extra_prop` for Light in WASM is its emissive brightness.
                    // The `lightBrightnessFactor` from floatColorToHex is how much the base color was scaled down if it was >1.
                    // We should use the `extra_prop` as the source of truth for the Light's brightness slider.
                    setDisplayBrightness(selectedObjMatProps.extra_prop);
                    break;
                default: // Diffuse or other
                    // Reset non-applicable fields to defaults
                    setDisplayRoughness(0.0);
                    setDisplayIor(1.5);
                    setDisplayBrightness(5.0); // Default brightness if not a light
                    break;
            }
        } else {
            // No object selected or props unavailable, reset to defaults
            setDisplayColor("#FFFFFF");
            setDisplayMaterialType(0);
            setDisplayIor(1.5);
            setDisplayRoughness(0.0);
            setDisplayBrightness(5.0);
        }
    }, [selectedObjMatProps]); // Re-run when selectedObjMatProps changes


    // Handlers for property changes - they call WASM
    // The UI will update reactively when selectedObjMatProps changes via the context
    const handleColorChange = (newColor: string) => {
        if (!isActuallyEditable) return;
        setDisplayColor(newColor); // Optimistic UI update
        wasmSetMaterialColor(newColor);
    };

    const handleMaterialTypeChange = (newType: number) => {
        if (!isActuallyEditable) return;
        setDisplayMaterialType(newType); // Optimistic UI update
        wasmSetMaterialType(newType);
    };

    const handleIorChange = (newIor: number) => {
        if (!isActuallyEditable || displayMaterialType !== 2) return;
        const clampedIor = Math.max(0.1, Math.min(5.0, newIor || 0.1));
        setDisplayIor(clampedIor); // Optimistic UI update
        wasmSetMaterialIOR(clampedIor);
    };

    const handleRoughnessChange = (newRoughness: number) => {
        if (!isActuallyEditable || displayMaterialType !== 1) return;
        const clampedRoughness = Math.max(0.0, Math.min(1.0, newRoughness || 0.0));
        setDisplayRoughness(clampedRoughness); // Optimistic UI update
        wasmSetMaterialRoughness(clampedRoughness);
    };

    const handleBrightnessChange = (newBrightness: number) => {
        if (!isActuallyEditable || displayMaterialType !== 3) return;
        const clampedBrightness = Math.max(0.1, Math.min(100.0, newBrightness || 0.1));
        setDisplayBrightness(clampedBrightness); // Optimistic UI update
        wasmSetMaterialBrightness(clampedBrightness);
    };


    
    return (
        <AccordionItem value="materialEditor">
            <AccordionTrigger 
                disabled={overallDisabled || !isActuallyEditable} 
                className={(overallDisabled || !isActuallyEditable) ? "cursor-not-allowed text-muted-foreground/70" : ""}
            >
                Material Editor {!(isActuallyEditable && !overallDisabled) && <span className="text-xs ml-2">(Not Editable)</span>}
            </AccordionTrigger>
            <AccordionContent className="space-y-4 pt-2">
                {/* Color Picker */}
                <div className="flex space-x-4">
                    <div className="flex-1 space-y-1">
                        <Label htmlFor="material-color" className="text-sm font-medium">Color</Label>
                        <Input
                            id="material-color"
                            type="color"
                            value={displayColor}
                            onChange={(e) => handleColorChange(e.target.value)}
                            className="w-full h-10 p-1"
                            disabled={overallDisabled || !isActuallyEditable}
                        />
                    </div>
                    {/* Material Type Dropdown */}
                    <div className="flex-1 space-y-1">
                        <Label htmlFor="material-type" className="text-sm font-medium">Material Type</Label>
                        <Select
                            value={displayMaterialType.toString()}
                            onValueChange={(value: string) => handleMaterialTypeChange(parseInt(value))}
                            disabled={overallDisabled || !isActuallyEditable}
                        >
                            <SelectTrigger id="material-type" className="h-10">
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
                {displayMaterialType === 1 && ( // Metal
                    <div className="space-y-1">
                        <Label htmlFor="material-roughness" className="text-xs text-muted-foreground">Roughness</Label>
                        <Input
                            id="material-roughness" type="number" value={displayRoughness}
                            onChange={(e) => handleRoughnessChange(parseFloat(e.target.value))}
                            step="0.01" min="0.0" max="1.0" className="w-full h-8 text-xs"
                            disabled={overallDisabled || !isActuallyEditable}
                        />
                    </div>
                )}
                {displayMaterialType === 2 && ( // Glass
                    <div className="space-y-1">
                        <Label htmlFor="material-ior" className="text-xs text-muted-foreground">Index of Refraction (IOR)</Label>
                        <Input
                            id="material-ior" type="number" value={displayIor}
                            onChange={(e) => handleIorChange(parseFloat(e.target.value))}
                            step="0.01" min="0.1" max="5.0" className="w-full h-8 text-xs"
                            disabled={overallDisabled || !isActuallyEditable}
                        />
                    </div>
                )}
                {displayMaterialType === 3 && ( // Light
                    <div className="space-y-1">
                        <Label htmlFor="material-brightness" className="text-xs text-muted-foreground">Brightness</Label>
                        <Input
                            id="material-brightness" type="number" value={displayBrightness}
                            onChange={(e) => handleBrightnessChange(parseFloat(e.target.value))}
                            step="0.1" min="0.1" max="100.0" className="w-full h-8 text-xs"
                            disabled={overallDisabled || !isActuallyEditable}
                        />
                    </div>
                )}
            </AccordionContent>
        </AccordionItem>
    );
};

export default MaterialEditorControls;