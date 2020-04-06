# VTracker
[![PyPI](https://img.shields.io/pypi/v/vtracker)](https://pypi.org/project/vtracker/)
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/vtracker)
[![codecov.io](https://codecov.io/github/aaronmussig/VTracker/coverage.svg?branch=master)](https://codecov.io/github/aaronmussig/VTracker?branch=master)

For tracking the relationship between group membership changes across versions.

## Installation
* PyPI: `pip install vtracker`

## Usage

1. Instantiate the `VTracker` class and specify the versions from oldest to newest.
2. Populate the tracker with each unique entity, and specify the state of the entity
at each of the versions. Missing versions have the state of 'Not Present'.

Consider the following example which generates the JSON required to display the
following D3 Sankey diagram similar to that of the 
[GTDB Taxon History](https://gtdb.ecogenomic.org/taxon_history/) tool:

```python
from vtracker import VTracker

vt = VTracker(('R80', 'R83', 'R86.2', 'R89', 'NCBI'))
vt.add('G000210735', {'R80': 's__Faecalibacterium prausnitzii_B',
                      'R83': 's__Faecalibacterium prausnitzii_B',
                      'R86.2': 's__Faecalibacterium prausnitzii_B',
                      'R89': 's__Faecalibacterium prausnitzii_G',
                      'NCBI': 's__Faecalibacterium prausnitzii'})

vt.add('G003287485', {'R89': 's__Faecalibacterium prausnitzii_G',
                      'NCBI': 's__Faecalibacterium prausnitzii'})

vt.add('G003287505', {'R89': 's__Faecalibacterium prausnitzii_G',
                      'NCBI': 's__Faecalibacterium prausnitzii'})

vt.add('G003293635', {'R89': 's__Faecalibacterium prausnitzii_G',
                      'NCBI': 's__Faecalibacterium prausnitzii'})

vt.add('G003508795', {'R80': 's__Faecalibacterium prausnitzii_B',
                      'R83': 's__Faecalibacterium prausnitzii_B',
                      'R86.2': 's__Faecalibacterium prausnitzii_B',
                      'R89': 's__Faecalibacterium prausnitzii_G'})
                      
sankey_json = vt.as_sankey_json()
```

![Sankey diagram example](https://raw.githubusercontent.com/aaronmussig/VTracker/master/docs/imgs/taxon_history.png)
