#[allow(dead_code)]
#[cfg(test)]
pub const GAMESTATE_JSON: &str = r###"{
    "campaign_name": "mp_Custodianship",
    "date": [
        2213,
        12,
        13
    ],
    "empires": [
        {
            "name": "Custodianship",
            "driver": {
                "Human": "Semantically_Invalid"
            },
            "budget": {
                "income": {
                    "Minerals": [
                        [
                            "country_base",
                            30.0
                        ],
                        [
                            "planet_miners",
                            34.896
                        ],
                        [
                            "orbital_mining_deposits",
                            25.3
                        ]
                    ],
                    "Physics": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_researchers",
                            26.304
                        ],
                        [
                            "orbital_research_deposits",
                            6.0
                        ]
                    ],
                    "Influence": [
                        [
                            "country_base",
                            3.0
                        ],
                        [
                            "country_ethic",
                            1.0
                        ],
                        [
                            "country_power_projection",
                            0.66164
                        ]
                    ],
                    "Unity": [
                        [
                            "country_base",
                            5.25
                        ],
                        [
                            "planet_jobs",
                            1.594
                        ],
                        [
                            "planet_bio_trophies",
                            43.308
                        ]
                    ],
                    "Energy": [
                        [
                            "country_base",
                            30.0
                        ],
                        [
                            "planet_technician",
                            132.32
                        ],
                        [
                            "orbital_mining_deposits",
                            19.8
                        ]
                    ],
                    "Engineering": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_researchers",
                            26.304
                        ],
                        [
                            "orbital_research_deposits",
                            6.0
                        ]
                    ],
                    "Food": [
                        [
                            "planet_farmers",
                            14.54
                        ]
                    ],
                    "ConsumerGoods": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_artisans",
                            15.44
                        ]
                    ],
                    "Alloys": [
                        [
                            "country_base",
                            5.0
                        ],
                        [
                            "planet_metallurgists",
                            37.056
                        ]
                    ],
                    "Society": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_researchers",
                            26.304
                        ],
                        [
                            "orbital_research_deposits",
                            5.0
                        ]
                    ]
                },
                "expense": {
                    "Food": [
                        [
                            "pop_category_bio_trophy",
                            9.0
                        ]
                    ],
                    "ConsumerGoods": [
                        [
                            "pop_category_bio_trophy",
                            9.0
                        ]
                    ],
                    "Energy": [
                        [
                            "ships",
                            10.45
                        ],
                        [
                            "ship_components",
                            12.9
                        ],
                        [
                            "station_gatherers",
                            6.0
                        ],
                        [
                            "station_researchers",
                            7.0
                        ],
                        [
                            "starbase_stations",
                            7.0
                        ],
                        [
                            "starbase_buildings",
                            1.0
                        ],
                        [
                            "starbase_modules",
                            2.0
                        ],
                        [
                            "planet_buildings",
                            19.0
                        ],
                        [
                            "planet_buildings_strongholds",
                            1.0
                        ],
                        [
                            "planet_districts",
                            9.0
                        ],
                        [
                            "planet_districts_cities",
                            10.0
                        ],
                        [
                            "planet_districts_industrial",
                            4.0
                        ],
                        [
                            "planet_researchers",
                            16.0
                        ],
                        [
                            "pop_category_drones",
                            46.2
                        ],
                        [
                            "armies",
                            6.0
                        ]
                    ],
                    "Minerals": [
                        [
                            "planet_metallurgists",
                            48.0
                        ],
                        [
                            "planet_artisans",
                            10.0
                        ]
                    ],
                    "Alloys": [
                        [
                            "ships",
                            2.15
                        ],
                        [
                            "ship_components",
                            2.365
                        ],
                        [
                            "planet_pop_assemblers",
                            4.4
                        ]
                    ],
                    "Unity": [
                        [
                            "leader_scientists",
                            10.0
                        ],
                        [
                            "leader_governors",
                            2.0
                        ]
                    ]
                },
                "balance": {
                    "Minerals": [
                        [
                            "country_base",
                            30.0
                        ],
                        [
                            "planet_miners",
                            34.896
                        ],
                        [
                            "planet_metallurgists",
                            -48.0
                        ],
                        [
                            "planet_artisans",
                            -10.0
                        ],
                        [
                            "orbital_mining_deposits",
                            25.3
                        ]
                    ],
                    "ConsumerGoods": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_artisans",
                            15.44
                        ],
                        [
                            "pop_category_bio_trophy",
                            -9.0
                        ]
                    ],
                    "Unity": [
                        [
                            "country_base",
                            5.25
                        ],
                        [
                            "planet_jobs",
                            1.594
                        ],
                        [
                            "planet_bio_trophies",
                            43.308
                        ],
                        [
                            "leader_scientists",
                            -10.0
                        ],
                        [
                            "leader_governors",
                            -2.0
                        ]
                    ],
                    "Energy": [
                        [
                            "country_base",
                            30.0
                        ],
                        [
                            "ships",
                            -10.45
                        ],
                        [
                            "ship_components",
                            -12.9
                        ],
                        [
                            "station_gatherers",
                            -6.0
                        ],
                        [
                            "station_researchers",
                            -7.0
                        ],
                        [
                            "starbase_stations",
                            -7.0
                        ],
                        [
                            "starbase_buildings",
                            -1.0
                        ],
                        [
                            "starbase_modules",
                            -2.0
                        ],
                        [
                            "planet_buildings",
                            -19.0
                        ],
                        [
                            "planet_buildings_strongholds",
                            -1.0
                        ],
                        [
                            "planet_districts",
                            -9.0
                        ],
                        [
                            "planet_districts_cities",
                            -10.0
                        ],
                        [
                            "planet_districts_industrial",
                            -4.0
                        ],
                        [
                            "planet_technician",
                            132.32
                        ],
                        [
                            "planet_researchers",
                            -16.0
                        ],
                        [
                            "pop_category_drones",
                            -46.2
                        ],
                        [
                            "orbital_mining_deposits",
                            19.8
                        ],
                        [
                            "armies",
                            -6.0
                        ]
                    ],
                    "Engineering": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_researchers",
                            26.304
                        ],
                        [
                            "orbital_research_deposits",
                            6.0
                        ]
                    ],
                    "Physics": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_researchers",
                            26.304
                        ],
                        [
                            "orbital_research_deposits",
                            6.0
                        ]
                    ],
                    "Alloys": [
                        [
                            "country_base",
                            5.0
                        ],
                        [
                            "ships",
                            -2.15
                        ],
                        [
                            "ship_components",
                            -2.365
                        ],
                        [
                            "planet_pop_assemblers",
                            -4.4
                        ],
                        [
                            "planet_metallurgists",
                            37.056
                        ]
                    ],
                    "Society": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_researchers",
                            26.304
                        ],
                        [
                            "orbital_research_deposits",
                            5.0
                        ]
                    ],
                    "Influence": [
                        [
                            "country_base",
                            3.0
                        ],
                        [
                            "country_ethic",
                            1.0
                        ],
                        [
                            "country_power_projection",
                            0.66164
                        ]
                    ],
                    "Food": [
                        [
                            "planet_farmers",
                            14.54
                        ],
                        [
                            "pop_category_bio_trophy",
                            -9.0
                        ]
                    ]
                },
                "income_last_month": {
                    "Physics": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_researchers",
                            26.304
                        ],
                        [
                            "orbital_research_deposits",
                            6.0
                        ]
                    ],
                    "Minerals": [
                        [
                            "country_base",
                            30.0
                        ],
                        [
                            "planet_miners",
                            34.896
                        ],
                        [
                            "orbital_mining_deposits",
                            25.3
                        ]
                    ],
                    "Society": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_researchers",
                            26.304
                        ],
                        [
                            "orbital_research_deposits",
                            5.0
                        ]
                    ],
                    "ConsumerGoods": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_artisans",
                            15.44
                        ]
                    ],
                    "Alloys": [
                        [
                            "country_base",
                            5.0
                        ],
                        [
                            "planet_metallurgists",
                            37.056
                        ]
                    ],
                    "Engineering": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_researchers",
                            26.304
                        ],
                        [
                            "orbital_research_deposits",
                            6.0
                        ]
                    ],
                    "Energy": [
                        [
                            "country_base",
                            30.0
                        ],
                        [
                            "planet_technician",
                            132.32
                        ],
                        [
                            "orbital_mining_deposits",
                            19.8
                        ]
                    ],
                    "Influence": [
                        [
                            "country_base",
                            3.0
                        ],
                        [
                            "country_ethic",
                            1.0
                        ],
                        [
                            "country_power_projection",
                            0.67174
                        ]
                    ],
                    "Food": [
                        [
                            "planet_farmers",
                            14.54
                        ]
                    ],
                    "Unity": [
                        [
                            "country_base",
                            5.25
                        ],
                        [
                            "planet_jobs",
                            1.594
                        ],
                        [
                            "planet_bio_trophies",
                            43.308
                        ]
                    ]
                },
                "expense_last_month": {
                    "Society": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_researchers",
                            26.304
                        ],
                        [
                            "orbital_research_deposits",
                            5.0
                        ]
                    ],
                    "Alloys": [
                        [
                            "country_base",
                            5.0
                        ],
                        [
                            "planet_metallurgists",
                            37.056
                        ]
                    ],
                    "Minerals": [
                        [
                            "country_base",
                            30.0
                        ],
                        [
                            "planet_miners",
                            34.896
                        ],
                        [
                            "orbital_mining_deposits",
                            25.3
                        ]
                    ],
                    "Food": [
                        [
                            "planet_farmers",
                            14.54
                        ]
                    ],
                    "Influence": [
                        [
                            "country_base",
                            3.0
                        ],
                        [
                            "country_ethic",
                            1.0
                        ],
                        [
                            "country_power_projection",
                            0.67174
                        ]
                    ],
                    "Energy": [
                        [
                            "country_base",
                            30.0
                        ],
                        [
                            "planet_technician",
                            132.32
                        ],
                        [
                            "orbital_mining_deposits",
                            19.8
                        ]
                    ],
                    "Physics": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_researchers",
                            26.304
                        ],
                        [
                            "orbital_research_deposits",
                            6.0
                        ]
                    ],
                    "Unity": [
                        [
                            "country_base",
                            5.25
                        ],
                        [
                            "planet_jobs",
                            1.594
                        ],
                        [
                            "planet_bio_trophies",
                            43.308
                        ]
                    ],
                    "Engineering": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_researchers",
                            26.304
                        ],
                        [
                            "orbital_research_deposits",
                            6.0
                        ]
                    ],
                    "ConsumerGoods": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_artisans",
                            15.44
                        ]
                    ]
                },
                "balance_last_month": {
                    "ConsumerGoods": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_artisans",
                            15.44
                        ]
                    ],
                    "Energy": [
                        [
                            "country_base",
                            30.0
                        ],
                        [
                            "planet_technician",
                            132.32
                        ],
                        [
                            "orbital_mining_deposits",
                            19.8
                        ]
                    ],
                    "Society": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_researchers",
                            26.304
                        ],
                        [
                            "orbital_research_deposits",
                            5.0
                        ]
                    ],
                    "Engineering": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_researchers",
                            26.304
                        ],
                        [
                            "orbital_research_deposits",
                            6.0
                        ]
                    ],
                    "Influence": [
                        [
                            "country_base",
                            3.0
                        ],
                        [
                            "country_ethic",
                            1.0
                        ],
                        [
                            "country_power_projection",
                            0.67174
                        ]
                    ],
                    "Physics": [
                        [
                            "country_base",
                            10.0
                        ],
                        [
                            "planet_researchers",
                            26.304
                        ],
                        [
                            "orbital_research_deposits",
                            6.0
                        ]
                    ],
                    "Minerals": [
                        [
                            "country_base",
                            30.0
                        ],
                        [
                            "planet_miners",
                            34.896
                        ],
                        [
                            "orbital_mining_deposits",
                            25.3
                        ]
                    ],
                    "Unity": [
                        [
                            "country_base",
                            5.25
                        ],
                        [
                            "planet_jobs",
                            1.594
                        ],
                        [
                            "planet_bio_trophies",
                            43.308
                        ]
                    ],
                    "Alloys": [
                        [
                            "country_base",
                            5.0
                        ],
                        [
                            "planet_metallurgists",
                            37.056
                        ]
                    ],
                    "Food": [
                        [
                            "planet_farmers",
                            14.54
                        ]
                    ]
                }
            },
            "resources": {
                "energy": 558.0887,
                "minerals": 2212.392,
                "food": 1247.72,
                "physics_research": 259.52768,
                "society_research": 247.632,
                "engineering_research": 253.632,
                "influence": 494.49584,
                "unity": 1750.93642,
                "consumer_goods": 3097.62,
                "alloys": 241.514,
                "volatile_motes": 0.0,
                "exotic_gases": 0.0,
                "rare_crystals": 0.0,
                "sr_living_metal": 0.0,
                "sr_zro": 0.0,
                "sr_dark_matter": 0.0
            }
        }
    ]
}"###;
