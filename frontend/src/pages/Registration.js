import { Input } from 'baseui/input';
import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Button, KIND as BKind } from "baseui/button";
import { useMutation, gql } from '@apollo/client';

import Loading from '../components/common/Loading';
import "./css/Auth.css"
import { useEffect } from 'react';
import { FormControl } from 'baseui/form-control';

const REGISTEG_QUERY = gql`
    mutation Register($login: String!, $email: String!, $password: String!) {
        users {
            register(login: $login, email: $email, password: $password)
        }
    }
`;

const Registration = () => {
    const navigate = useNavigate();

    useEffect(() => {
        if (localStorage.getItem('token') !== null) {
            navigate('/');
        }
    })

    const [error, setError] = useState(null);
    const [formData, setFormData] = useState({
        login: "",
        email: "",
        password: "",
        password2: "",
    });

    const success = () => {
        navigate('/login');
    }

    const error_notification = (error) => {
        setError(error);
        console.log(error);
    }

    const [register, { loading }] = useMutation(REGISTEG_QUERY, { onCompleted: success, onError: error_notification });

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

    const validate_password = (password) => {
        if (password.length < 8) {
            return false;
        }
        return true;
    }

    const password_should_match = () => {
        return formData.password === formData.password2;
    }

    const onChange = (e) => {
        setFormData({ ...formData, [e.target.name]: e.target.value });
    }

    const invalid_email = formData.email !== "" && !validate_email(formData.email);
    const invalid_password = formData.password !== "" && !validate_password(formData.password);
    const invalid_password_match = !password_should_match();
    const registration_disabled = invalid_email || invalid_password || invalid_password_match || formData.login === "" || formData.email === "" || formData.password === "";

    const submit = () => {
        if (registration_disabled) {
            return;
        }
        register({ variables: { login: formData.login, email: formData.email, password: formData.password } });
        setFormData({
            login: "",
            email: "",
            password: "",
            password2: "",
        });
        setError(null);
    }

    return <div className='wrapper'>
        <h1 className='title'>Registration</h1>
        {error && <p className='error'>{error.message}</p>}
        <div className='form-wrapper'>
            <FormControl>
                <Input
                    name='login'
                    placeholder='Login'
                    onChange={e => onChange(e)}
                    required
                />
            </FormControl>
            <FormControl error={invalid_email ? 'Invalid email' : null}>
                <Input
                    name='email'
                    placeholder='Email'
                    onChange={e => onChange(e)}
                    required
                    error={invalid_email}
                />
            </FormControl>

            <FormControl error={invalid_password ? 'Password should be at least 8 characters long' : null}>

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
                    error={invalid_password}
                />
            </FormControl>
            <FormControl error={invalid_password_match ? 'Passwords should match' : null}>

                <Input

                    placeholder='Repeat Password'
                    name='password2'
                    type="password"
                    overrides={{
                        MaskToggleHideIcon: () => '🙈',
                        MaskToggleShowIcon: () => '🐵',
                    }}
                    onChange={e => onChange(e)}
                    required
                    error={invalid_password_match}
                />
            </FormControl>
            <Button style={{ width: "100%" }} onClick={submit} kind={BKind.secondary} disabled={registration_disabled} >Register</Button>
        </div>
    </div>
};

export default Registration;