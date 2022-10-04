
import React from "react";

import Select from 'react-select';
import ResourceSummary from "./ResourceSummary";

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

interface ResourceSelectionProps {
    on_change: (options: string[]) => void
}

const ResourceSelection = (props: ResourceSelectionProps) => (
    <Select options={options}
        isMulti
        onChange={(e) => {
            let opts = e.map((e) => e.value);
            props.on_change(opts)
        }} />
)


interface ResourceSummaryContainerProps {
    campaign_name: string,
    empire_name: string
}
interface ResourceSummaryContainerState {
    selection: string[]
}

class ResourceSummaryContainer extends React.Component<ResourceSummaryContainerProps, ResourceSummaryContainerState> {



    onChange = (options: string[]) => {
        this.setState({ selection: options })
    }
    render() {
        if (this.state && this.state.selection && this.state.selection.length > 0) {
            return <div>
                <ResourceSelection on_change={this.onChange} />
                <ResourceSummary campaign_name={this.props.campaign_name} empire_name={this.props.empire_name} resources={this.state.selection} />
            </div>
        } else return <div><ResourceSelection on_change={this.onChange} /></div>
    }

}




export default ResourceSummaryContainer;