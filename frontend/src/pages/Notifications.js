// src/components/Alerts.js

import { useQuery, gql, useMutation } from '@apollo/client';
import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import Loading from '../components/common/Loading';
import './css/Notifications.css';
import {
    Card,
    StyledBody,
    StyledAction,
} from "baseui/card";
import { Button } from 'baseui/button';
import Price from '../components/common/Price';
import TextMsg from '../components/common/TextMsg';
import { ButtonGroup } from 'baseui/button-group';
import CreateNotification from '../components/notification/Create';
import TelegramAuthButton from '../components/common/TelegramAuthButton';

const LOAD_NOTIFICATIONS = gql`
    query LoadNotifications($token: String!) {
        notification {
            all(token: $token) {
                id
                coinName
                source
                changeType
                changeValue
                currentPrice
                name
            }
        }
    }
`;

const DELETE_NOTIFICATION = gql`
    mutation DeleteNotification($token: String!, $id: Int!) {
        notification {
            remove(token: $token, id: $id)
        }
    }
`;

const Notifications = () => {
    const navigate = useNavigate();
    const [token, setToken] = useState(null);
    const { loading, error, data, refetch } = useQuery(LOAD_NOTIFICATIONS, {
        variables: { token }, skip: token === null || token === undefined, notifyOnNetworkStatusChange: true
    });
    const [deleteNotification] = useMutation(DELETE_NOTIFICATION);

    React.useEffect(() => {
        const updateToken = () => {
            const token = localStorage.getItem('token');
            setToken(token);
            if (token === undefined || token === null) {
                navigate('/login');
            }
        }

        window.addEventListener('storage', updateToken);
        updateToken();
    }, [navigate]);


    if (loading || data === undefined) return <Loading />;
    if (error) return <p>Error: {error.message}</p>;

    const notifications = data.notification.all;

    const remove = (id) => {
        deleteNotification({
            variables: { token, id: parseInt(id) }, onCompleted: () => {
                refetch();
            }
        });

    };

    return (
        <React.Fragment>
            <h1 className='header'>
                Notifications
                <CreateNotification callback={() => {
                    refetch();
                }} />
                <TelegramAuthButton />
            </h1>
            <div className='list'>
                {
                    notifications.map((notification) => {
                        const valuePrefix = notification.changeType === 'by Percent' ? '%' : '$';
                        return <Card key={notification.id} overrides={{
                            Root: {
                                style: {
                                    backgroundColor: '#333',
                                    minWidth: '350px',
                                    width: '10%',
                                }
                            }
                        }} title={notification.name} >

                            <StyledBody>
                                <div className='items'>
                                    <TextMsg text={notification.coinName} tooltip={'Asset'} />
                                    <TextMsg text={notification.source} tooltip={'Source'} />
                                    <TextMsg text={notification.changeValue + valuePrefix} tooltip={'Change ' + notification.changeType} />
                                    <Price price={notification.currentPrice} tooltip={'On Price'} float_point={3} ></Price>

                                </div>
                                <div className='items'>
                                </div>
                            </StyledBody>
                            <StyledAction>
                                <div className='items'>
                                    <ButtonGroup overrides={{
                                        Root: {
                                            style: {
                                                width: '100%',
                                            }
                                        }
                                    }}>
                                        <Button overrides={{ BaseButton: { style: { width: '100%' } } }} onClick={() => remove(notification.id)}>Remove</Button>
                                        < CreateNotification callback={() => { refetch() }} editElem={notification} >
                                            <Button overrides={{ BaseButton: { style: { width: '100%' } } }}>Edit</Button>
                                        </CreateNotification>

                                    </ButtonGroup>
                                </div>
                            </StyledAction>

                        </Card>
                    })
                }

            </div >
        </React.Fragment >
    );
};

export default Notifications;
