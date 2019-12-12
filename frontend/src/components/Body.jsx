import React from 'react';
import Output from "./Output";
import '../styles/Body.css';

class Body extends React.Component {

    render() {
        const {results} = this.props;
        return (
            <div className="output-section">
                {results.map((result, i) => (
                    <div className="out">
                        <div><button className="del-out" onClick={() => this.props.onDelete(i)}>delete</button></div>
                        <button className="output-btn" onClick={() => this.props.openPopup(result.processes)}>
                            <Output
                                method={result.method}
                                processes={result.processes}
                                interval={result.interval}
                                pageFaultRate={result.avg_page_miss}
                                algorithm={result.algorithm}
                                no_processes={result.no_processes}
                                frames={result.no_frames}
                            />
                        </button>
                    </div>
                ))}
            </div>
        )
    }
}

export default Body;