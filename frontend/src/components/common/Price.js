import React, { useEffect } from "react";
import { Card, StyledBody, StyledRoot } from 'baseui/card';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faArrowTrendUp, faArrowTrendDown } from '@fortawesome/free-solid-svg-icons';

const Price = ({ price, tooltip, isPriceUp, color, size, float_point, minWidth }) => {
    const [priceState, setPriceState] = React.useState(price);
    const [isPriceUpState, setIsPriceUpState] = React.useState(isPriceUp);
    const float_point_prop = float_point || 5;

    useEffect(() => {
        if (price > priceState) {
            setIsPriceUpState(true);
        }
        else if (price < priceState) {
            setIsPriceUpState(false);
        }
        setPriceState(price);

    }, [price]);


    const prop_size = size || 'auto';
    const color_prop = color ||
        (isPriceUpState != undefined ? (isPriceUpState ? '#52c41a' : '#FE5F55') : "primary");

    return (
        <Card
            overrides={{
                Root: {
                    style: {
                        width: prop_size, height: prop_size, background: '#333', minWidth: minWidth + 'px'
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
                    color: color_prop, fontWeight: 'bold',
                }} >
                    {parseFloat(priceState).toFixed(float_point_prop)}
                    {isPriceUpState != undefined && <FontAwesomeIcon icon={isPriceUpState == true ? faArrowTrendUp : faArrowTrendDown} />}
                </div>
            }
        </Card >
    );
}

export default Price;