from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="cutters",
    version="0.1.3",
    description="A rule based sentence segmentation library.",
    long_description=open("README.md", "r", encoding="utf-8").read(),
    long_description_content_type="text/markdown",
    author="cyanic-selkie",
    author_email="cyanic-selkie@protonmail.com",
    url="https://github.com/cyanic-selkie/cutters",
    license="MIT",
    rust_extensions=[
        RustExtension("cutters.cutters", binding=Binding.PyO3, debug=False)
    ],
    classifiers=[
        "Natural Language :: Croatian",
        "Natural Language :: English",
        "Topic :: Text Processing",
    ],
    zip_safe=False,
)
