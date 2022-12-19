import pytest
from mizu import Mizu

import asyncio


@pytest.mark.asyncio
async def test_parse():
    m = Mizu(loop_=asyncio.get_running_loop())
    assert await m.aioparse("# hello") == "<h1>hello</h1>\n"