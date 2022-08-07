from setuptools import setup
from setuptools_rust import Binding, RustExtension


def call_setup():
    setup(
        name="uwb_serial",
        version="0.0.1",
        description="Using rust serial on Python like PySerial",
        # long_description=open("README.rst", encoding="utf-8").read(),
        # long_description_content_type="text/x-rst",
        keywords="rust serial",
        author="Seok Won",
        author_email="ikr@kakao.com",
        maintainer="Seok Won",
        maintainer_email="ikr@kakao.com",
        python_requires=">=3.7",
        license="MIT",
        rust_extensions=[RustExtension("uwb_serial", binding=Binding.PyO3)],
        classifiers=[
            "Development Status :: 3 - Alpha",
            "Intended Audience :: Developers",
            "License :: OSI Approved :: MIT License",
            "Operating System :: OS Independent",
            "Programming Language :: Python :: 3",
            "Programming Language :: Python :: 3.5",
            "Programming Language :: Python :: 3.6",
            "Programming Language :: Python :: 3.7",
            "Programming Language :: Python :: 3.8",
            "Programming Language :: Python :: 3.9",
            "Programming Language :: Rust",
        ],
        zip_safe=False,
    )


if __name__ == "__main__":
    call_setup()
