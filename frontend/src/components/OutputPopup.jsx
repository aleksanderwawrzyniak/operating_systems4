import React from 'react';
import Programme from "./Programme";
import '../styles/OutputPopup.css';


class OutputPopup extends React.Component {

    render() {
        const {data} = this.props;
        console.log(data);
        return (
            <div className="popup">
                <button className="exit" onClick={this.props.closePopup}>+</button>
                <div className="output-popup">
                    {data.map(programme => (
                        <Programme self={programme}/>
                    ))}
                </div>
            </div>
        )
    }
}

export default OutputPopup;