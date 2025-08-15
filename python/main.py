import httpimport
import sys
import os


def import_remote(module_url):
    with httpimport.remote_repno(module_url):
        import hello
    hello.hello()

    mod = httpimport.load('hello', url=module_url)
    mod.hello()

def get_httpimport_logger():
    import logging
    return logging.getLogger("httpimport")

def set_httpimport_log_level_from_env():
    new_level = os.environ.get(
        "HTTPIMPORT_LOGLEVEL",
        "WARNING"
    )
    new_level = "DEBUG" if "-v" in sys.argv else new_level
    set_httpimport_log_level(new_level)

def set_httpimport_log_level(new_level):
    logger = get_httpimport_logger()
    logger.setLevel(new_level)
    for h in logger.handlers:
        h.setLevel(new_level)

set_httpimport_log_level_from_env()
