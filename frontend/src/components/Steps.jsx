import React from 'react';
import Step from "./Step";
import '../styles/Steps.css';


class Steps extends React.Component {
    state = {
        steps: this.props.steps,
        stepsToShow: [],
        isButtonActive: true,
        output: ""
    };

    scrollDown = (value) => {
        let left = window.pageXOffset || document.documentElement.scrollLeft;
        let top = window.pageYOffset || document.documentElement.scrollTop
        window.scrollTo(left, top + value);
    };

    addStep = (e) => {
        let toShowLength = this.state.stepsToShow.length;
        if (this.state.stepsToShow.length < this.state.steps.length) {
            this.setState({
                stepsToShow: [
                    ...this.state.stepsToShow,
                    this.state.steps[this.state.stepsToShow.length],
                ],
                output: this.state.steps[toShowLength].pages
            });
        } else {
            this.setState({
                isButtonActive: false
            });
        }

        if (window.innerHeight - e.clientY <= 35) {
            this.scrollDown((window.innerHeight - e.clientY) + 13);
        }
    };


    render() {
        const {stepsToShow} = this.state;
        return (
            <div>
                <div className="out-pages">{this.state.output}</div>
                {stepsToShow.map(step => (
                    <Step step={step.algorithm_step} percentage={step.fault_rate}/>
                ))}
                <div>
                    <button className="button" type="button" disabled={!this.state.isButtonActive}
                            onClick={this.addStep}>next step
                    </button>
                </div>
            </div>
        )

    }
}

export default Steps;