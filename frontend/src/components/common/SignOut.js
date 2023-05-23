import { gql, useMutation } from '@apollo/client';
import React from 'react';
import { Link } from 'react-router-dom';

const SIGNOUT_MUTATION = gql`
    mutation logout($token: String!) {
        users {
            logout(token:$token)
        }
    }
`;

const SignOut = () => {
    const success = () => {
        localStorage.removeItem('token');
        localStorage.removeItem('expiresAt');
        window.dispatchEvent(new Event('storage'));
    }


    const [signout] = useMutation(SIGNOUT_MUTATION, {
        variables: { token: localStorage.getItem('token') },
        onCompleted: success
    });


    const submit = () => {
        signout();
    }

    return (
        <Link onClick={submit}>SignOut</Link>
    )
};

export default SignOut;