import { Spinner } from 'baseui/spinner';
import * as React from 'react';
import { useStyletron } from 'baseui';

const Loading = () => {
    const [css, theme] = useStyletron();
    return (
        <div className={
            css({
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                color: 'white',
                marginTop: '5%',
            })}
        >
            <Spinner $size={100}> </Spinner>
            <h2>Loading...</h2>
        </div >
    );
}

export default Loading;