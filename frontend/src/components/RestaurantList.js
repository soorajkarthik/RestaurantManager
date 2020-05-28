import React from 'react';
import Restaurant from './Restaurant';
import ReactLoading from "react-loading"

import "../css/RestaurantList.css"

class RestaurantList extends React.Component {
    constructor(props) {
        super(props);
        this.state = { data: null, text: props.text }
    }

    componentDidMount() {
        this.fetchNewData("");
    }

    componentDidUpdate(oldProps) {
        if (this.props.text !== oldProps.text) {
            this.setState({ text: this.props.text });
            this.fetchNewData(this.props.text);
        }
    }

    fetchNewData(text) {
        fetch(`http://localhost:8000/${text}`)
            .then(response => response.json())
            .then(data => this.setState({ data: data.value }))
    }

    render() {
        if (this.state.data === null) {
            return (
                <div>
                    <ReactLoading className="Loading" type={"bars"} color={"#282c34"} height={"30%"} width={"30%"} />
                </div>
            );
        }
        else if (this.state.data.length === 0) {
            return (
                <div className="NoneFound">
                    <h1>No Restaurants Found</h1>
                    <h3>I couldn't find any reviews for the restaurant you searched for. Click submit and be the first to review it!</h3>
                </div>
            )
        }
        else {
            return (
                <div className="ListHolder">
                    <div className="ListTitle">
                        <h1>Results</h1>
                    </div>
                    {this.state.data.map(restaurant => <Restaurant key={restaurant.name} name={restaurant.name} avgRating={restaurant.avgRating} numReviews={restaurant.ratings.length} />)}
                </div>
            );
        }
    }
}

export default RestaurantList;