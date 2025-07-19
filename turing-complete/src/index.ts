export enum Reg {
    reg0 = 0,
    reg1,
    reg2,
    jmpcmp = 3,
    reg3 = 3,
    reg4,
    reg5,
    in = 0b110,
    out = 0b110,
    io = 0b110,
}

export enum Cnd {
    TRUE = 0b100,
    FALSE = 0,

    EQ = 1,
    NE = 0b101,

    GT = 0b111,
    GTE = 0b110,

    LT = 0b011,
    LTE = 0b010,
}

export enum AOp {
    OR = 0,
    NAND,
    NOR,
    AND,
    ADD,
    SUB,
    // unused0,
    // unused1,
}

export type Label = string;

export type Ins =
    | { t: "immediate"; v: number | Label }
    | { t: "copy"; from: Reg; to: Reg }
    | { t: "jmp"; cond: Cnd }
    | { t: "compute"; op: AOp };

// deprecated
const I = (x: Ins) => {
    if (x.t == "immediate" && x.v > 63) throw new Error("invalid immediate");
    return x;
};

const mov = (from: Reg, to: Reg) => I({ t: "copy", from, to });
const imm = (v: number) => I({ t: "immediate", v });
const jmp = (cond: Cnd) => I({ t: "jmp", cond });
const alu = (op: AOp) => I({ t: "compute", op });

function assemble(syntax: TemplateStringsArray, ...values: Ins[]) {
    let labels = {};
    const encode = (ins: Ins) => {
        let b = ["immediate", "compute", "copy", "jmp"].indexOf(ins.t) << 6;

        if (ins.t == "immediate" && typeof ins.v == "string") ins.v = labels[ins.v] - 1;
        if (ins.t == "immediate") b |= ins.v;
        if (ins.t == "copy") b |= (ins.from << 3) | ins.to;
        if (ins.t == "jmp") b |= ins.cond;
        if (ins.t == "compute") b |= ins.op;
        return { b, ins };
    };

    let ip = 0;
    return syntax
        .map((txt, i) => {
            let label: string;
            if ((label = txt.match(/\n(\w+):\n/m)?.[1]!)) {
                labels[label] = ip;
            }
            // console.log(i, JSON.stringify(txt), values[i]);
            if (values[i]) {
                ip++;
                return { ...encode(values[i]), label };
            }
        })
        .filter((x) => !!x);
}

const log = (a: ReturnType<typeof assemble>) =>
    console.log(
        a
            .map(
                ({ ins, b, label }) =>
                    `${label ? `#${label}:\n` : ""}${b}\t# ${ins.t} ${{
                        immediate: () => ins.v,
                        copy: () =>
                            [ins.from, ins.to].map((x) => Reg[x]).join("→"),
                        jmp: () => Cnd[ins.cond],
                        compute: () => AOp[ins.op],
                    }[ins.t]?.()}`
            )
            .join("\n")
    );

const clog = (a: ReturnType<typeof assemble>) =>
    console.log(JSON.stringify(a.map((x) => x.b)));

const check = (a: number[], b: ReturnType<typeof assemble>) => {
    if (!a.every((x, i) => b[i].b == x)) {
        console.error("test failed");
        process.exit(1);
    }
};

check(
    [177, 5, 130, 68, 158],
    assemble`
${I({ t: "copy", from: Reg.io, to: Reg.reg1 })}
${I({ t: "immediate", v: 5 })}
${I({ t: "copy", from: Reg.reg0, to: Reg.reg2 })}
${I({ t: "compute", op: AOp.ADD })}
${I({ t: "copy", from: Reg.reg3, to: Reg.io })}
`
);

// ALU: reg3 = reg1 (ins.op) reg2
// IMM: reg0 = ins.val
// JMP: if (reg3 ins.check) ip = reg0
//
//reg0..3 - clobber

// CALIBRATING LASER CANNONS
// reg5 - accumulator (=0)
//reg4 - loop idx countdown
check(
    [180, 169, 6, 130, 68, 157, 161, 1, 130, 69, 156, 1, 197, 174],
    assemble`
init:
    ${mov(Reg.in, 4)}
loop:
    ${mov(5, 1)}
    ${imm(6)} ${mov(0, 2)}
    ${alu(AOp.ADD)}
    ${mov(3, 5)}

    ${mov(4, 1)}
    ${imm(1)} ${mov(0, 2)}
    ${alu(AOp.SUB)}
    ${mov(3, 4)}

    ${imm(2 - 1)} ${jmp(Cnd.NE)}
end:
    ${mov(5, Reg.out)}
`
);

// SPACIAL INVASION
// ALU: reg3 = reg1 (ins.op) reg2
// IMM: reg0 = ins.val
// JMP: if (reg3 ins.check) ip = reg0
//
//reg0..3 - clobber

export enum Robot {
    left = 0,
    fwd,
    right,
    noop,
    action,
    shoot,
}

log(assemble`
turn_left:
    ${imm(Robot.left)} ${mov(0, Reg.out)}
    ${imm(Robot.left)} ${mov(0, Reg.out)}
    ${imm(Robot.fwd)}  ${mov(0, Reg.out)}
    ${imm(Robot.right)} ${mov(0, Reg.out)}
try_fwd:
    ${imm(Robot.fwd)}  ${mov(0, Reg.out)}
    ${mov(Reg.in, Reg.jmpcmp)}
    ${imm("try_fwd")} ${jmp(Cnd.EQ)}
`);

// log(assemble`
// go_bottomleft:
//     ${imm(Robot.left)} ${mov(0, Reg.out)}
//     ${imm(Robot.left)} ${mov(0, Reg.out)}
//     ${imm(Robot.fwd)}  ${mov(0, Reg.out)}
//     ${imm(Robot.right)} ${mov(0, Reg.out)}
// init:
//     ${imm(16)} ${mov(0, 4)} i=16
// loop:
//     ${mov(4, 1)}
//     ${imm(1)} ${mov(0, 2)}
//     ${alu(AOp.SUB)}
//     ${mov(3, 4)} i-- (could be moved after the jmp to opt)

//     ${imm(Robot.fwd)}  ${mov(0, Reg.out)}
//     ${imm(10)} ${jmp(Cnd.NE)}
// `)
