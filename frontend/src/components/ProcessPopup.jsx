import React from 'react';
import Process from "./Process";
import '../styles/ProcessPopup.css';

class ProcessPopup extends React.Component {

    handleChange = (id, value) => {
        this.props.onChange(id, value);
    };

    handleDelete = (id) => {
        this.props.onDelete(id);
    };

    render() {
        const {programs} = this.props;
        return (
            <div className="back">
                <button className="close-btn" onClick={this.props.closePopup}>+</button>
                <div className="processes">
                    {programs.map(program => (
                        <Process size={program.size} id={program.id} requests={program.requests}
                                 onChange={this.handleChange} onDelete={this.handleDelete}/>
                    ))}
                </div>
            </div>
        )
    }
}

export default ProcessPopup;