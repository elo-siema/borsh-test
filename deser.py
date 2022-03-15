
from tokenize import Token
from borsh_construct import CStruct, U64, Bool, Option, Enum
from anchorpy import borsh_extension

AccountState = Enum("Uninitialized", "Initialized", "Frozen", enum_name="AccountState")

ACCOUNT_SCHEMA = CStruct(
    "state" / AccountState,
    "state2" / AccountState,
)

f = open("file.borsh", "rb").read()

parsed = ACCOUNT_SCHEMA.parse(f)
print(parsed)
print(isinstance(parsed.state, AccountState.enum.Uninitialized))
print(isinstance(parsed.state2, AccountState.enum.Uninitialized))