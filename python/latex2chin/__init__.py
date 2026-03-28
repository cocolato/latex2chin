import os
from dataclasses import dataclass, field
from latex2chin.latex2chin import parse_latex
from langchain_openai import ChatOpenAI
from langchain.agents import create_agent
from langchain_core.tools import tool


@dataclass
class _Settings:
    api_key: str = field(default_factory=lambda: os.getenv("LATEX2CHIN_API_KEY", ""))
    base_url: str = field(
        default_factory=lambda: os.getenv(
            "LATEX2CHIN_BASE_URL", "https://api.deepseek.com"
        )
    )
    model: str = field(
        default_factory=lambda: os.getenv("LATEX2CHIN_MODEL", "deepseek-chat")
    )


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
        {
            "messages": [
                {
                    "role": "system",
                    "content": (
                        "你是一个将文本中LaTeX数学表达式转换为中文读法的助手。\n\n"
                        "工作流程：\n"
                        "1. 定位输入文本中所有LaTeX数学表达式，包括：\n"
                        "   - 含反斜杠命令的片段（如 \\frac{1}{2}、\\sqrt{x}、\\sum 等）\n"
                        "   - 上下标符号组合（如 x^2、a_1）\n"
                        "2. 对每个识别到的LaTeX片段，必须单独调用 latex_to_chinese 工具转换，不得自行翻译\n"
                        "3. 用工具返回的中文读法替换对应片段，其余文本逐字保留，不得增删\n"
                        "4. 你的回复必须且只能是替换后的完整文本本身，禁止输出任何前缀、解释、过渡句或总结"
                    ),
                },
                {
                    "role": "user",
                    "content": f"待处理文本：\n{string}",
                },
            ]
        }
    )

    return res["messages"][-1].content


__all__ = ["parse_latex", "parse_chinese", "configure", "settings"]
