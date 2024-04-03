class ControlWrapper:
    thrust: float
    elevator: float
    aileron: float
    rudder: float

    def __init__(self, thrust: float, elevator: float, aileron: float, rudder: float):
        ...


class StateWrapper:
    npos: float
    epos: float
    altitude: float
    phi: float
    theta: float
    psi: float
    velocity: float
    alpha: float
    beta: float
    p: float
    q: float
    r: float

    def __init__(self, npos: float, epos: float, altitude: float, phi: float, theta: float,
                 psi: float, velocity: float, alpha: float, beta: float, p: float, q: float, r: float): ...


class StateExtendWrapper:
    nx: float
    ny: float
    nz: float
    mach: float
    qbar: float
    ps: float

    def __init__(self, nx: float, ny: float, nz: float,
                 mach: float, qbar: float, ps: float): ...


class CoreOutputWrapper:
    state: StateWrapper
    control: ControlWrapper
    state_extend: StateExtendWrapper

    def __init__(self, state: StateWrapper, control: ControlWrapper,
                 state_extend: StateExtendWrapper): ...


class PlaneMessageWrapper:
    id: UuidWrapper
    time: float
    output: CoreOutputWrapper


class PluginInfoWrapper:
    name: str
    author: str
    version: str
    description: str

    def __init__(self, name: str, author: str,
                 version: str, description: str): ...


class PluginStateWrapper:
    @staticmethod
    def enable() -> PluginStateWrapper: ...

    @staticmethod
    def disable() -> PluginStateWrapper: ...

    @staticmethod
    def failed() -> PluginStateWrapper: ...


class PluginInfoTupleWrapper:
    id: str
    info: PluginInfoWrapper | None
    state: PluginStateWrapper

    def __init__(self, id: str, state: PluginStateWrapper,
                 info: PluginInfoWrapper | None) -> None: ...


class UuidWrapper:
    @staticmethod
    def new_v4() -> UuidWrapper:
        ...

    @staticmethod
    def parse_str(s: str) -> UuidWrapper:
        ...


class TrimTargetWrapper:
    altitude: float
    velocity: float

    def __init__(self, altitude: float, velocity: float) -> None: ...


class TrimInitWrapper:
    control: ControlWrapper
    alpha: float

    def __init__(self, control: ControlWrapper, alpha: float): ...


class NelderMeadOptionWrapper:
    max_fun_evals: int
    max_iter: int
    tol_fun: float
    tol_x: float

    def __init__(self, max_fun_evals: int, max_iter: int,
                 tol_fun: float, tol_x: float) -> None: ...


class FlightConditionWrapper:
    @staticmethod
    def from_str(s: str) -> FlightConditionWrapper: ...

    @staticmethod
    def wings_level() -> FlightConditionWrapper: ...

    @staticmethod
    def turning() -> FlightConditionWrapper: ...

    @staticmethod
    def pull_up() -> FlightConditionWrapper: ...

    @staticmethod
    def roll() -> FlightConditionWrapper: ...


class PlaneInitCfgWrapper:
    deflection: list[float] | None
    trim_target: TrimTargetWrapper
    trim_init: TrimInitWrapper | None
    flight_condition: FlightConditionWrapper | None
    optim_options: NelderMeadOptionWrapper | None

    def __init__(self, trim_target: TrimTargetWrapper,
                 deflection: list[float] | None,
                 trim_init: TrimInitWrapper | None,
                 flight_condition: FlightConditionWrapper | None,
                 optim_options: NelderMeadOptionWrapper | None): ...


class PyClient:
    @staticmethod
    async def new(host: str, port: int) -> PyClient: ...

    async def stop(self): ...
    
    def tick(self, tick_period: int | None): ...

    async def get_model_infos(self) -> list[PluginInfoTupleWrapper]: ...

    async def push_plane(
        self, arg: tuple[UuidWrapper, PlaneInitCfgWrapper | None]) -> UuidWrapper: ...

    async def send_control(self, arg: tuple[UuidWrapper, ControlWrapper | None]): ...

    async def output(self) -> PlaneMessageWrapper: ...

    async def lost_plane(self) -> str: ...

    async def new_plane(self) -> str: ...

    async def error(self) -> str: ...
