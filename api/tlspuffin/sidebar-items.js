initSidebarItems({"constant":[["GIT_MSG",""],["GIT_REF",""]],"macro":[["define_signature",""],["mutator",""],["nyi_fn",""],["term",""],["term_arg",""]],"mod":[["agent","[`Agent`]s represent communication participants like Alice, Bob or Eve. Attackers are usually not represented by these [`Agent`]s. Attackers are represented through a recipe term (see [`InputAction`])."],["algebra","The term module defines typed[`Term`]sof the form `fn_add(x: u8, fn_square(y: u16)) → u16`. Each function like `fn_add` or `fn_square` has a shape. The variables `x` and `y` each have a type. These types allow type checks during the runtime of the fuzzer. These checks restrict how[`Term`]scan be mutated in the fuzzer module."],["debug",""],["error",""],["experiment",""],["extraction",""],["fuzzer","The fuzzer module setups the fuzzing loop. It also is responsible for gathering feedback from runs and restarting processes if they crash."],["graphviz","This module adds plotting capabilities to[`Term`]sand Traces. The output of the functions in this module can be passed to the command line utility `dot` which is part of graphviz."],["io","These are currently implemented by using an in-memory buffer. One might ask why we want two channels. There two very practical reasons for this. Note that these are advantages for the implementation and are not strictly required from a theoretical point of view."],["log",""],["put","Generic [`PUT`] trait defining an interface with a TLS library with which we can:"],["put_registry",""],["static_certs",""],["tcp",""],["tls","The tls module provides concrete implementations for the functions used in the term. The module offers a variety of [`DynamicFunction`]s which can be used in the fuzzing."],["trace","This module contains [`Trace`]s consisting of several [`Step`]s, of which each has either an [`OutputAction`] or [`InputAction`]. This is a declarative way of modeling communication between [`Agent`]s. The [`TraceContext`] holds data, also known as [`VariableData`], which is created by [`Agent`]s during the concrete execution of the Trace. It also holds the [`Agent`]s with the references to concrete PUT."],["variable_data","Definition of the VariableData trait. A VariableData can contain any data which has a `'static` type. This is true for [`rustls::msgs::message::Message`] for example."]]});