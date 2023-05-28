import React from "react";
import { Card } from 'baseui/card';

const TextMsg = ({ text, tooltip, size, minWidth, backgroundColor }) => {

    const prop_size = size || 'auto';
    const backgroundColor_prop = backgroundColor || '#333';


    return (
        <Card
            overrides={{
                Root: {
                    style: {
                        width: prop_size, height: prop_size, backgroundColor: backgroundColor_prop, minWidth: minWidth + 'px'
                    }
                },
                Title: {
                    style: () => ({
                        fontSize: '1em',
                        opacity: 0.5,
                        marginBottom: '-0.25em',
                    })
                }
            }}
            title={tooltip || undefined}
        >
            {
                <div style={{
                    fontWeight: 'bold',
                }} >
                    {text}
                </div>
            }
        </Card >
    );
}

export default TextMsg;