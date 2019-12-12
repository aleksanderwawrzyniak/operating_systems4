import React from 'react';
import '../styles/Step.css';

class Step extends React.Component {

    findStepType = (c) => {
        let ret = 'output-node';
        if (c === '$') {
            ret += ' output-miss-node';
        }
        return ret;
    };

    findNodeType = (str) => {
        let ret = 'page';

        // ||x||
        if (/\|\|(X|\d+)\|\|/.test(str)) {
            ret += ' miss-page';
            return ret;
        }

        // _|x|_
        if (/_\|(X|\d+)\|_/.test(str)) {
            ret += ' hit-aim-page';
            return ret;
        }

        // |x|
        if (/\|(X|\d+)\|/.test(str)) {
            ret += ' aim-page';
            return ret;
        }

        // _x_
        if (/_(X|\d+)_/.test(str)) {
            ret += ' hit-page';
            return ret;
        }

        return ret;
    }

    render() {
        const {step, percentage} = this.props;
        let splitted = step.trim().split(' ');
        return (
            <div>
                <div className={this.findStepType(splitted[1])}>
                    <span className="page ref">{splitted[0].replace(/\D/g, '')}</span>
                    <span className="page">=></span>
                    {splitted.map((str, index) => (
                        index > 1 ? (
                            <span className={this.findNodeType(str)}>{str.replace(/[^X\d]/g, '')}</span>
                        ) : (
                            null
                        )
                    ))}
                    <span className="percentage">{Math.round(percentage * 100)}%</span>
                </div>
                {/*<PercentageBar value={Math.round(percentage * 100)}/>*/}
            </div>
        )
    }
}

export default Step;