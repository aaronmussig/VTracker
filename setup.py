import os
import re

from setuptools import setup


def read_version():
    path = os.path.join(os.path.abspath(os.path.dirname(__file__)), 'vtracker/__init__.py')
    with open(path, 'r') as fh:
        return re.search(r'__version__\s?=\s?[\'"](.+)[\'"]', fh.read()).group(1)


def readme():
    with open('README.md') as f:
        return f.read()


setup(name='vtracker',
      version=read_version(),
      description='For tracking the relationship between group membership changes across versions.',
      long_description=readme(),
      long_description_content_type='text/markdown',
      author='Aaron Mussig',
      author_email='aaronmussig@gmail.com',
      url='https://github.com/aaronmussig/VTracker',
      license='GPL3',
      project_urls={
          "Bug Tracker": "https://github.com/aaronmussig/VTracker/issues",
          "Documentation": "https://github.com/aaronmussig/VTracker",
          "Source Code": "https://github.com/aaronmussig/VTracker",
      },
      classifiers=[
          'Development Status :: 5 - Production/Stable',
          'Intended Audience :: Science/Research',
          'License :: OSI Approved :: GNU General Public License v3 (GPLv3)',
          'Natural Language :: English',
          'Operating System :: OS Independent',
          'Programming Language :: Python :: 2',
          'Programming Language :: Python :: 3',
          'Topic :: Scientific/Engineering',
      ],
      keywords='track relationship group membership version',
      packages=['vtracker'],
      install_requires=['typing'],
      python_requires='>=2.7',
      data_files=[("", ["LICENSE"])]
      )
