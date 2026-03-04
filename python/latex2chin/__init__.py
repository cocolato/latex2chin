import os
from dataclasses import dataclass, field
from latex2chin.latex2chin import parse_latex
from langchain_openai import ChatOpenAI
from langchain.agents import create_agent
from langchain_core.tools import tool


@dataclass
class _Settings:
    api_key: str = field(default_factory=lambda: os.getenv("LATEX2CHIN_API_KEY", ""))
    base_url: str = field(default_factory=lambda: os.getenv("LATEX2CHIN_BASE_URL", "https://api.deepseek.com"))
    model: str = field(default_factory=lambda: os.getenv("LATEX2CHIN_MODEL", "deepseek-chat"))


settings = _Settings()


def configure(
    *,
    api_key: str | None = None,
    base_url: str | None = None,
    model: str | None = None,
):
    if api_key is not None:
        settings.api_key = api_key
    if base_url is not None:
        settings.base_url = base_url
    if model is not None:
        settings.model = model


@tool
def latex_to_chinese(latex_string: str) -> str:
    """Convert a LaTeX math expression to Chinese text."""
    print("latex_to_chinese called with:", latex_string)
    return parse_latex(latex_string)


def parse_chinese(
    string: str,
    *,
    api_key: str | None = None,
    base_url: str | None = None,
    model: str | None = None,
) -> str:
    llm = ChatOpenAI(
        model=model or settings.model,
        base_url=base_url or settings.base_url,
        api_key=api_key or settings.api_key,
    )

    agent = create_agent(llm, [latex_to_chinese])

    res = agent.invoke(
        {"messages": [{"role": "user", "content": "将下面这段话里的符合latex语法规范的数学表达式和latex转换为中文,要记住,数学表达式也当作latex处理,初次之外的文本保持不变: " + string}]}
    )

    return res["messages"][-1].content


__all__ = ["parse_latex", "parse_chinese", "configure", "settings"]

