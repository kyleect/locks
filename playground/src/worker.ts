import init, { loxRun, locksDisassemble, loxDiagnose } from 'lox-wasm';

onmessage = async (event) => {
  await init();

  // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
  const { code, action } = event.data;

  switch (action) {
    case 'run':
      loxRun(code as string);
      break;

    case 'disassemble':
      locksDisassemble(code as string);
      break;

    case 'diagnose':
      loxDiagnose(code as string);
      break;

    default:
      break;
  }
};
