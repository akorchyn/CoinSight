import React, { useState } from "react";

import "./Search.css"

const Search = () => {
    const [value, setValue] = useState("");
    const onInput = (e) => setValue(e.target.value);


    const searchToggle = (e) => {
        const searchWrapper = document.querySelector(".search-wrapper");
        const inputHolder = document.querySelector(".input-holder");
        if (searchWrapper.classList.contains("active") && inputHolder.hasAttributes()) {
            console.log("hello")
            searchWrapper.classList.remove("active");
            setValue("");
        } else if (!searchWrapper.classList.contains("active")) {
            console.log("bye")
            searchWrapper.classList.add("active");
            e.preventDefault();
        }
    };

    return (
        <div className="search-wrapper">
            <div className="input-holder">
                <input type="text" className="search-input" placeholder="Type to search" onInput={onInput} value={value} />
                <button className="search-icon" onClick={searchToggle}><span></span></button>
            </div>
            <span className="close" onClick={searchToggle}></span>
        </div>
    );
}

export default Search;
