FROM python:3
WORKDIR /src
COPY ./ .

RUN pip3 install ortools
RUN pip3 install cpmpy
RUN pip3 install z3-solver

CMD python /src/aoc.py
