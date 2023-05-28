import React, { useEffect, useState } from "react"
import './Create.css'
import { Select } from "baseui/select";
import Loading from "../common/Loading";
import { useQuery, gql, useMutation } from "@apollo/client";
import { Input } from "baseui/input";
import CryptoCurrencyCard from "../common/CryptoCurrencyCard";
import { Modal, ModalBody, ModalFooter, ModalHeader, ModalButton } from "baseui/modal";
import { Link } from "react-router-dom";

const CRYPTO_QUERY = gql`
    query Search {
        crypto {
            all {
                id
                name
                symbol
                latestAggregatedPrice {
                    id
                    medianPrice
                }
            }
        }
    }  
`;

const SOURCE_QUERY = gql`
    query Source {
        source {
            all {
                id
                name
            }
        }
    }
`

const NOTIFICATION_MUTATION = gql`
    mutation Create($token: String!, $name: String!, $coinName: String!, $source: String!, $changeType: String!, $changeValue: String!, $currentPrice: String!) {
        notification {
            create(token: $token, name: $name, coinName: $coinName, source: $source, changeType: $changeType, changeValue:$changeValue, currentPrice: $currentPrice)
        }
    }
`;

const EDIT_NOTIFICATION_MUTATION = gql`
    mutation Edit($token: String!, $id: Int!, $name: String!, $coinName: String!, $source: String!, $changeType: String!, $changeValue: String!, $currentPrice: String!) {
        notification {
            edit(token: $token, id: $id, name: $name, coinName: $coinName, source: $source, changeType: $changeType, changeValue:$changeValue, currentPrice: $currentPrice)
        }
    }
`;

const CreateNotification = ({ preCryptoSymbol, callback, editElem, children }) => {
    const [token, setToken] = useState(null);
    const [isOpen, setIsOpen] = useState(false);
    const [cryptoSymbol, setCryptoSymbol] = useState(undefined);
    const [notificationType, setNotificationType] = useState(undefined);
    const [source, setSource] = useState(undefined);
    const [value, setValue] = useState(0);
    const [name, setName] = useState(undefined);
    const [errorMsg, setError] = useState(undefined);
    const [isEditPrepared, setIsEdit] = useState(false);

    // Graphql
    const { loading, error, data } = useQuery(CRYPTO_QUERY);
    const { loading: sourceLoading, error: sourceError, data: sourceData } = useQuery(SOURCE_QUERY);
    const [createNotification] = useMutation(NOTIFICATION_MUTATION);
    const [editNotification] = useMutation(EDIT_NOTIFICATION_MUTATION);



    useEffect(() => {
        const updateToken = () => {
            const token = localStorage.getItem('token');
            setToken(token);
            if (!token) {
                setIsOpen(false);
            }
        }

        window.addEventListener('storage', updateToken);
        updateToken();
    }, []);



    if (loading || sourceLoading) return <Loading />;
    if (error || sourceError) return <p>Error: Failed to load</p>;

    const created = () => {
        setIsOpen(false);
        setCryptoSymbol(undefined);
        setNotificationType(undefined);
        setSource(undefined);
        setValue(0);
        setName(undefined);
        setError(undefined);

        if (callback) {
            callback();
        }
    }

    const create = () => {

        if (editElem) {
            editNotification({
                variables: {
                    token: token,
                    id: editElem.id,
                    coinName: cryptoSymbol[0].name,
                    source: source[0].name,
                    changeType: notificationType[0].id,
                    changeValue: value.toString(),
                    currentPrice: cryptoSymbol[0].latestAggregatedPrice.medianPrice,
                    name,
                }, onCompleted: created, onError: (error) => setError(error.message)
            })

        } else {
            createNotification({
                variables: {
                    token: token,
                    coinName: cryptoSymbol[0].name,
                    source: source[0].name,
                    changeType: notificationType[0].id,
                    changeValue: value.toString(),
                    currentPrice: cryptoSymbol[0].latestAggregatedPrice.medianPrice,
                    name,
                }, onCompleted: created, onError: (error) => setError(error.message)
            });
        }
    };

    const cryptos = data.crypto.all;
    if (preCryptoSymbol && cryptoSymbol === undefined) {
        const preCrypto = cryptos.find(crypto => crypto.symbol === preCryptoSymbol);
        if (preCrypto) {
            setCryptoSymbol([preCrypto]);
        }
    }
    const selectedCryptoData = cryptoSymbol?.[0];

    const startEnhancer = notificationType?.[0]?.id === 'by Percent' ? '%' : '$';

    const source_options = {
        'CoinSight analytics': [
            { name: 'Median Price' },
            { name: 'Third Quartile Price' },
            { name: 'First Quartile Price' }
        ],
        Sources: sourceData.source.all,
    }

    const is_ready = token && cryptoSymbol && notificationType && source && value && name;
    const titleText = editElem ? 'Edit notification' : 'Create notification';

    if (editElem && !isEditPrepared) {
        setCryptoSymbol([cryptos.find(crypto => crypto.name === editElem.coinName)]);
        setNotificationType([{ id: editElem.changeType }]);
        setSource([{ name: editElem.source }]);
        setValue(editElem.changeValue);
        setName(editElem.name);
        setIsEdit(true);
    }

    return (
        <React.Fragment>
            {
                token && children &&
                <div onClick={() => setIsOpen(true)} style={{ width: '100%', height: '100%' }}>
                    {children}
                </div>
            }
            {
                token && !children &&
                <img src='/notification_icon.svg' className='notifyIcon' onClick={() => setIsOpen(true)} />

            }

            <Modal onClose={() => setIsOpen(false)} isOpen={isOpen} overrides={{
                Dialog: {
                    style: {
                        backgroundColor: '#333'
                    }
                },
            }}>
                <ModalHeader>{titleText}</ModalHeader>
                <ModalBody>
                    {
                        errorMsg &&
                        <h3 className="error">{errorMsg}</h3>
                    }
                    <Input placeholder="Enter notification name" value={name} required onChange={(e) => setName(e.target.value)} />
                    <br />
                    <Select
                        options={cryptos}
                        required
                        placeholder="Select cryptocurrency"
                        value={cryptoSymbol}
                        labelKey="name"
                        valueKey="id"
                        onChange={(params) => {
                            setCryptoSymbol(params.value)
                        }}
                    />
                    <br />

                    {
                        selectedCryptoData &&
                        <React.Fragment>
                            <Link to={`/asset/${selectedCryptoData.symbol}`} style={{ textDecoration: 'none' }} >
                                <CryptoCurrencyCard symbol={selectedCryptoData.symbol} assetName={selectedCryptoData.name} currentPrice={selectedCryptoData.latestAggregatedPrice.medianPrice} />
                            </Link>
                            <br />
                        </React.Fragment>


                    }
                    <Select options={[{ id: 'by Value' }, { id: 'by Percent' }]}
                        placeholder="Select notification type"
                        labelKey="id"
                        value={notificationType}
                        required onChange={(e) => setNotificationType(e.value)}
                        disabled={cryptoSymbol === undefined}

                    />
                    <br />

                    <Input placeholder="Enter value" type="float" required
                        value={value} onChange={(e) => setValue(e.target.value)}
                        startEnhancer={startEnhancer}
                        disabled={cryptoSymbol === undefined || notificationType === undefined} />
                    <br />

                    <Select options={source_options} placeholder="Select source for notification"
                        disabled={cryptoSymbol == undefined}
                        valueKey='name' labelKey='name' required
                        onChange={(params) => setSource(params.value)}
                        value={source} />
                </ModalBody>
                <ModalFooter>
                    <ModalButton onClick={() => setIsOpen(false)}>
                        Cancel
                    </ModalButton>
                    <ModalButton onClick={create} disabled={!is_ready}>{editElem ? 'Edit' : 'Create'}</ModalButton>
                </ModalFooter>
            </Modal>
        </React.Fragment >
    )

}

export default CreateNotification
