import React, { useState } from 'react';
import {
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
} from "@/components/ui/accordion";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Switch } from "@/components/ui/switch";
import { Minus, Plus } from 'lucide-react';

// No props needed if all state and handlers are local
interface TransformControlsProps { }

// Mock WASM function calls (replace with your actual WASM calls)
const wasmTranslateObject = (axis: 'x' | 'y' | 'z', delta: number) => {
    console.log(`WASM: Translate object on axis ${axis} by ${delta}`);
    // Example: Module.ccall('translate_selected_object', null, ['string', 'number'], [axis, delta]);
};

const wasmRotateObject = (axis: 'x' | 'y' | 'z', delta: number) => {
    console.log(`WASM: Rotate object on axis ${axis} by ${delta}`);
    // Example: Module.ccall('rotate_selected_object', null, ['string', 'number'], [axis, delta]);
};

const wasmScaleObjectUniformly = (delta: number) => {
    console.log(`WASM: Scale object uniformly by ${delta}`);
    // Example: Module.ccall('scale_selected_object_uniformly', null, ['number'], [delta]);
};

const wasmSetSnapToGrid = (enabled: boolean) => {
    console.log(`WASM: Set snap to grid to ${enabled}`);
    // Example: Module.ccall('set_snap_to_grid_enabled', null, ['boolean'], [enabled]);
}

// A small helper to render a row of controls (+/- buttons for an axis)
const AxisControlRow: React.FC<{
    axisLabel: string;
    onDecrement: () => void;
    onIncrement: () => void;
    disabled?: boolean; // Added disabled prop
}> = ({ axisLabel, onDecrement, onIncrement, disabled }) => (
    <div className="flex items-center space-x-2 py-1">
        <Label htmlFor={`transform-${axisLabel.toLowerCase()}`} className={`w-5 text-xs uppercase ${disabled ? 'text-muted-foreground/50' : 'text-muted-foreground'}`}>{axisLabel}</Label>
        <Button 
            id={`transform-${axisLabel.toLowerCase()}-decrement`} 
            variant="outline" 
            size="icon" 
            className="h-8 w-8" 
            onClick={onDecrement}
            disabled={disabled} // Apply disabled state
        >
            <Minus className="h-4 w-4" />
        </Button>
        <Button 
            id={`transform-${axisLabel.toLowerCase()}-increment`} 
            variant="outline" 
            size="icon" 
            className="h-8 w-8" 
            onClick={onIncrement}
            disabled={disabled} // Apply disabled state
        >
            <Plus className="h-4 w-4" />
        </Button>
    </div>
);

const TransformControls: React.FC<TransformControlsProps> = () => {
    // Local state for increments
    const [positionIncrement, setPositionIncrement] = useState<number>(0.1);
    const [rotationIncrement, setRotationIncrement] = useState<number>(5); // Degrees
    const [scaleIncrement, setScaleIncrement] = useState<number>(0.05);
    const [followCursorEnabled, setFollowCursorEnabled] = useState<boolean>(false);

    // Local event handlers that call WASM functions
    const handleTranslate = (axis: 'x' | 'y' | 'z', direction: 1 | -1) => {
        wasmTranslateObject(axis, positionIncrement * direction);
    };

    const handleRotate = (axis: 'x' | 'y' | 'z', direction: 1 | -1) => {
        wasmRotateObject(axis, rotationIncrement * direction);
    };

    const handleScaleUniformly = (direction: 1 | -1) => {
    };

    const handleToggleFollowCursor = (checked: boolean) => {
        setFollowCursorEnabled(checked);
        console.log(`Follow cursor: ${checked}`);
    }

    return (
        <AccordionItem value="transformAndSnap">
            <AccordionTrigger>Transform</AccordionTrigger>
            <AccordionContent className="space-y-4 pt-2">
                {/* Position Controls */}
                <div className="space-y-2">
                    <Label className="text-sm font-medium">Position</Label>
                    <div className="flex items-center justify-between py-1">
                        <Label htmlFor="follow-cursor-switch" className="text-sm font-medium">
                            Follow Cursor
                        </Label>
                        <Switch
                            id="follow-cursor-switch"
                            checked={followCursorEnabled}
                            onCheckedChange={handleToggleFollowCursor}
                        />
                    </div>
                    <AxisControlRow
                        axisLabel="X"
                        onDecrement={() => handleTranslate('x', -1)}
                        onIncrement={() => handleTranslate('x', 1)}
                        disabled={followCursorEnabled} // Pass disabled state
                    />
                    <AxisControlRow
                        axisLabel="Y"
                        onDecrement={() => handleTranslate('y', -1)}
                        onIncrement={() => handleTranslate('y', 1)}
                        disabled={followCursorEnabled} // Pass disabled state
                    />
                    <AxisControlRow
                        axisLabel="Z"
                        onDecrement={() => handleTranslate('z', -1)}
                        onIncrement={() => handleTranslate('z', 1)}
                        disabled={followCursorEnabled} // Pass disabled state
                    />
                    <div className="flex items-center justify-between pt-1">
                        <Label htmlFor="pos-increment" className="text-xs text-muted-foreground">Increment</Label>
                        <Input
                            id="pos-increment"
                            type="number"
                            value={positionIncrement}
                            onChange={(e) => setPositionIncrement(parseFloat(e.target.value) || 0.1)}
                            step="0.01"
                            min="0.01"
                            className="w-20 h-6 text-xs"
                        />
                    </div>
                </div>

                {/* Rotation Controls */}
                <div className="space-y-2">
                    <Label className="text-sm font-medium">Rotation</Label>
                    <AxisControlRow
                        axisLabel="X"
                        onDecrement={() => handleRotate('x', -1)}
                        onIncrement={() => handleRotate('x', 1)}
                    />
                    <AxisControlRow
                        axisLabel="Y"
                        onDecrement={() => handleRotate('y', -1)}
                        onIncrement={() => handleRotate('y', 1)}
                    />
                    <AxisControlRow
                        axisLabel="Z"
                        onDecrement={() => handleRotate('z', -1)}
                        onIncrement={() => handleRotate('z', 1)}
                    />
                    <div className="flex items-center justify-between pt-1">
                        <Label htmlFor="rot-increment" className="text-xs text-muted-foreground">Increment (°)</Label>
                        <Input
                            id="rot-increment"
                            type="number"
                            value={rotationIncrement}
                            onChange={(e) => setRotationIncrement(parseFloat(e.target.value) || 1)}
                            step="1"
                            min="1"
                            className="w-20 h-6 text-xs"
                        />
                    </div>
                </div>
                
                {/* Scale (Uniform) Controls */}
                <div className="space-y-2">
                    <Label className="text-sm font-medium">Scale (Uniform)</Label>
                    <div className="flex items-center space-x-2 py-0">
                        <Label htmlFor="transform-scale-decrement" className="w-5 text-xs uppercase text-muted-foreground"></Label> 
                        <Button id="transform-scale-decrement" variant="outline" size="icon" className="h-6 w-6" onClick={() => handleScaleUniformly(-1)}>
                            <Minus className="h-4 w-4" />
                        </Button>
                        <Button id="transform-scale-increment" variant="outline" size="icon" className="h-6 w-6" onClick={() => handleScaleUniformly(1)}>
                            <Plus className="h-4 w-4" />
                        </Button>
                    </div>
                    <div className="flex items-center justify-between pt-1">
                        <Label htmlFor="scale-increment" className="text-xs text-muted-foreground">Increment</Label>
                        <Input
                            id="scale-increment"
                            type="number"
                            value={scaleIncrement}
                            onChange={(e) => setScaleIncrement(parseFloat(e.target.value) || 0.01)}
                            step="0.01"
                            min="0.01"
                            className="w-20 h-6 text-xs"
                        />
                    </div>
                </div>

                {/* Divider (Optional, can be removed if sections are distinct enough) */}
                <div className="border-t border-border my-3"></div>
            </AccordionContent>
        </AccordionItem>
    );
};

export default TransformControls;