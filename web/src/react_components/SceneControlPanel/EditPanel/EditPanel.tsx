import React from 'react';
import { Accordion } from "@/components/ui/accordion";
import TransformControls from './TransformControls';
import MaterialEditorControls from './MaterialEditorControls';
// import SnapSettingsControls from './SnapSettingsControls';
// import MaterialEditorControls from './MaterialEditorControls';

const EditPanel: React.FC = () => {
    return (
        // This div wrapper is fine. The Accordion inside will manage its items.
        <div className="space-y-0"> {/* Reduced outer spacing if it's inside another content area */}
            <Accordion type="multiple" defaultValue={['transform']} className="w-full">
                <TransformControls />
                <MaterialEditorControls />
            </Accordion>
        </div>
    );
};

export default EditPanel;