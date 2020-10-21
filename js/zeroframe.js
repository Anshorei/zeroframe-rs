export function cmd(cmd, params={}) {
    console.log('sending cmd', cmd, params)
    document.frame.cmd(cmd, params, res => {
        console.log('received result', res)
    })
}

export async function cmdp(cmd, params={}) {
    console.log('sending cmdp', cmd, params);
    let result = await document.frame.cmdp(cmd, params)
    console.log('received result', result)
    return result
}
