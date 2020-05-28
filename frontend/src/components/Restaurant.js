import React from 'react';
import "../css/Restaurant.css"

function Restaurant(props) {
    return (
        <div className="Holder">
                <p>{props.name}</p>
                <p>Rating: {props.avgRating.toFixed(1)}</p>
                <p>Number of Reviews: {props.numReviews}</p>
        </div>
    )
}

export default Restaurant;