
import React from "react";
import ResourceSummary from "./ResourceSummary";

import Select from 'react-select';

const options = [
    { value: 'Energy', label: 'Energy' },
    { value: 'Minerals', label: 'Minerals' },
    { value: 'Food', label: 'Food' },
    { value: 'Physics', label: 'Physics' },
    { value: 'Society', label: 'Society' },
    { value: 'Engineering', label: 'Engineering' },
    { value: 'Influence', label: 'Influence' },
    { value: 'Unity', label: 'Unity' },
    { value: 'ConsumerGoods', label: 'Consumer Goods' },
    { value: 'Alloys', label: 'Alloys' },
    { value: 'Motes', label: 'Motes' },
    { value: 'Gasses', label: 'Gasses' },
    { value: 'Crystals', label: 'Crystals' },
    { value: 'LivingMetal', label: 'Living Metal' },
    { value: 'Zro', label: 'Zro' },
    { value: 'DarkMatter', label: 'Dark Matter' },
    { value: 'Nanites', label: 'Nanites' }
]

const MyComponent = ({ oc: updateResources }) => (
    <Select options={options}
        isMulti
        onChange={(e) => {
            let opts = e.map((e) => e.value);
            updateResources(opts)
        }} />
)

class ResourceSummaryContainer extends React.Component {

    constructor(props) {
        super(props);
        this.state = { selection: [] };

    }

    onChange = (options) => {
        this.setState({ selection: options })
    }
    render() {
        if (this.props.empire_name && this.props.campaign_name) {
            if (this.state.selection.length > 0) {
                return <div>
                    <MyComponent oc={this.onChange} />
                    <ResourceSummary campaign_name={this.props.campaign_name} empire_name={this.props.empire_name} resources={this.state.selection} />

                </div>
            } else {

                return <div>
                    <MyComponent oc={this.onChange} />

                </div>
            }

        } else {
            return <></>
        }
    }
}




export default ResourceSummaryContainer;