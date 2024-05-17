# %%
import networkx as nx
import numpy as np
import pandas as pd
import random
import polars as pl
import threading
import concurrent.futures
import datetime
from multiprocessing import Pool

# %%
pl.DataFrame()

# %%
def random_strg(size:int):
    strg  = list()
    for i in range(size):
        n = np.random.randint(65,90)
        w = chr(n)
        strg.append(w)
        
    return ''.join(strg)

# %%
def random_float(start,end): 
    return round( random.random() * random.randrange(start,end),2 )


# %%
def str_dataframe(rows, cols): 
    data = dict()
    for i in range(cols):
        name = f'str_col{i}'
        values = [random_strg(np.random.randint(1,30) ) for i in range(rows)]
        data.update({name:values})

    return pl.DataFrame(data)
   

# %%
def float_dataframe(rows, cols): 
    data = dict()
    for i in range(cols):
        name = f'float_col{i}'
        values = [random_float(i,55_000) for i in range(rows)]
        data.update({name:values})

    return pl.DataFrame(data)

# %%
def gen_fake_dataframe(rows, cols):
    return pl.concat([str_dataframe(rows,cols),float_dataframe(rows,cols) ],how='horizontal')

# %%
def task(n):
    print('started...',n)
    gen_fake_dataframe(10_000,15 ).write_avro(f'./test_{n}.avro',name="teste")
    print('done...',n)

if __name__ == '__main__':
    with Pool() as ex:   
        ex.map(task,range(10))
        
     



