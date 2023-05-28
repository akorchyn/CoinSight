import { Input } from 'baseui/input';
import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Button, KIND as BKind } from "baseui/button";
import { useMutation, gql } from '@apollo/client';
import { FormControl } from 'baseui/form-control';

import Loading from '../components/common/Loading';
import "./css/Auth.css"
import { useEffect } from 'react';

const LOGIN_QUERY = gql`
    mutation login($email: String!, $password: String!) {
        users {
            login(email: $email, password: $password) {
                token
                expiresAt
            }
        }
    }
`;

const Login = () => {
    const navigate = useNavigate();

    useEffect(() => {
        if (localStorage.getItem('token') !== null) {
            navigate('/');
        }
    })


    const [error, setError] = useState(null);
    const [formData, setFormData] = useState({
        email: "",
        password: "",
    });

    const success = (data) => {
        const token = data.users.login.token;
        const expiresAt = data.users.login.expiresAt;
        localStorage.setItem('token', token);
        localStorage.setItem('expiresAt', expiresAt);
        window.dispatchEvent(new Event('storage'));
        navigate(-1);
    }

    const error_notification = (error) => {
        setError(error);
    }

    const [login, { loading }] = useMutation(LOGIN_QUERY, { onCompleted: success, onError: error_notification });

    if (loading) {
        return <Loading />;
    }

    const validate_email = (email) => {
        const i = email.search('@');
        if (i === 0 || i === email.length - 1 || i === -1) {
            return false;
        }
        return true;
    }

    const onChange = (e) => {
        setFormData({ ...formData, [e.target.name]: e.target.value });
    }

    const invalid_email = formData.email !== "" && !validate_email(formData.email);
    const login_disabled = invalid_email || formData.login === "" || formData.email === "" || formData.password === "";

    const submit = () => {
        if (login_disabled) {
            return;
        }
        login({ variables: { email: formData.email, password: formData.password } });
        setFormData({
            email: "",
            password: "",
        });
        setError(null);
    }

    return <div className='wrapper'>
        <h1 className='title'>Login</h1>
        {error && <p className='error'>{error.message}</p>}
        <div className='form-wrapper'>
            <FormControl error={invalid_email ? 'Invalid email' : null}>
                <Input
                    name='email'
                    placeholder='Email'
                    onChange={e => onChange(e)}
                    required
                    error={invalid_email}
                />
            </FormControl>

            <br />

            <Input
                placeholder='Password'
                name='password'
                type="password"

                overrides={{
                    MaskToggleHideIcon: () => '🙈',
                    MaskToggleShowIcon: () => '🐵',
                }}
                onChange={e => onChange(e)}
                required
            />
            <br />
            <Button style={{ width: "100%" }} onClick={submit} kind={BKind.secondary} disabled={login_disabled} >Login</Button>
        </div>
    </div >
};

export default Login;