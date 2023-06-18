import React, { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useMutation, gql } from '@apollo/client';
import Loading from './Loading';
import { Modal, ModalBody, ModalFooter, ModalHeader, ModalButton } from 'baseui/modal';

const START_TELEGRAM_AUTH = gql`
  mutation StartTelegramAuth($token: String!) {
    users {
        startTelegramAuth(token: $token) 
    }
  }
`;

function TelegramAuthButton() {
    const navigate = useNavigate();
    const [code, setCode] = useState(null);
    const [modalOpen, setModalOpen] = useState(false);
    const [token, setToken] = useState(null);
    const [startAuth, { loading, error }] = useMutation(START_TELEGRAM_AUTH, {
        variables: { token },
        onCompleted: (data) => {
            setCode(data.users.startTelegramAuth);
            setModalOpen(true);
        },
    });

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


    if (loading) return <Loading />;
    if (error) return <p>Error</p>;

    return (
        <React.Fragment>
            <img style={{ width: '30px', height: '30px', cursor: "pointer" }} onClick={() => startAuth()} src="/Telegram_logo.svg.webp" ></img>

            <Modal
                onClose={() => setModalOpen(false)}
                closeable
                isOpen={modalOpen}
                animate
                autoFocus

            >
                <ModalHeader>Telegram settings modal</ModalHeader>
                <ModalBody>
                    <p>You can link your telegram to the website using this code: <b>{code}</b></p>
                    <p>Send this <i><b>/verify {code}</b></i> to the <Link style={{ textDecoration: 'none' }} target='_blank' to={`https://t.me/coin_sight_bot?text=/verify%20${code}`}>@coin_sight_bot</Link> telegram bot</p>
                </ModalBody>
                <ModalFooter>
                    <ModalButton onClick={() => setModalOpen(false)}>Okay</ModalButton>
                </ModalFooter>
            </Modal >

        </React.Fragment >
    );
}

export default TelegramAuthButton;
