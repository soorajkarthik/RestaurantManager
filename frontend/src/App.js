import React from 'react';
import DebounceInput from 'react-debounce-input';
import './css/App.css';
import RestaurantList from './components/RestaurantList.js';
import Header from './components/Header.js'
import Footer from './components/Footer.js'

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = { text: '', rating: 0 };
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  render() {
    return (
      <div>
        <Header className="App-header" />

        <div className="Form-holder">
          <form onSubmit={this.handleSubmit} className="Form">
            <label className="Form-label">
              Search for restaurant:
                <DebounceInput className="Form-text" type="text" debounceTimeout={500} onChange={(event) => this.setState({ text: event.target.value })} />
            </label>

            <select className="Form-select" value={this.state.rating} onChange={(event) => this.setState({ rating: parseInt(event.target.value, 10) })}>
              <option value="0">Zero</option>
              <option value="1">One</option>
              <option value="2">Two</option>
              <option value="3">Three</option>
              <option value="4">Four</option>
              <option value="5">Five</option>
            </select>

            <input className="Form-submit" type="submit" value="Submit" />
          </form>
        </div>

        <RestaurantList text={this.state.text} />
        <Footer />
      </div>
    )
  }

  handleSubmit() {

    fetch('http://localhost:8000/new', {
      method: "post",
      body: JSON.stringify({
        name: this.state.text,
        rating: this.state.rating
      }),
      headers: {
        "Content-Type": "application/json",
      },
    });
  }
}

export default App;
