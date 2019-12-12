import React from 'react';
import '../styles/StartingSection.css';

class StartingSection extends React.Component {

    render() {
        return (
            <div className="start">
                <div className="selects">
                    <div className="select-field">
                        <div>Paging algorithm select</div>
                        <select className="my-select" onChange={(event) => this.props.setAlgorithm(event.target.value)}>
                            <option value="lru">LRU</option>
                            <option value="fifo">FIFO</option>
                            <option value="alru">ALRU</option>
                            <option value="opt">OPT</option>
                            <option value="rand">RAND</option>
                        </select>
                    </div>
                    <div className="select-field">
                        <div>Frame allocaton method</div>
                        <select className="my-select" onChange={(event) => this.props.setMethod(event.target.value)}>
                            <option value="equal">Equal</option>
                            <option value="proportional">Proportional</option>
                            <option value="random">Random</option>
                            <option value="pff">Page Fault Frequency</option>
                            <option value="wsa">Working Set</option>
                        </select>
                    </div>
                </div>
                <div>
                    <button className="start-btn" type="button" onClick={this.props.startSimulation}>Start Simulation
                    </button>
                </div>
            </div>
        )
    }
}

export default StartingSection;