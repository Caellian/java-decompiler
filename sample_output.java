/*
 * decompiler development test
 */
package org.apache.commons.exec;

import org.apache.commons.exec.ExecuteResultHandler;
import org.apache.commons.exec.ExecuteException;

public class DefaultExecuteResultHandler implements ExecuteResultHandler {
private static int SLEEP_TIME_MS;
private boolean hasResult;
private int exitValue;
private ExecuteException exception;
public DefaultExecuteResultHandler() {
/* aload_0 */
/* invokespecial b7 0 */
/* aload_0 */
/* iconst_0 */
/* putfield b5 0 */
/* aload_0 */
/* ldc 12 */
/* putfield b5 0 */
/* return */
}

public void onProcessComplete(int arg_0) {
/* aload_0 */
/* iload_1 */
/* putfield b5 0 */
/* aload_0 */
/* aconst_null */
/* putfield b5 0 */
/* aload_0 */
/* iconst_1 */
/* putfield b5 0 */
/* return */
}

public void onProcessFailed(ExecuteException arg_0) {
/* aload_0 */
/* aload_1 */
/* invokevirtual b6 0 */
/* putfield b5 0 */
/* aload_0 */
/* aload_1 */
/* putfield b5 0 */
/* aload_0 */
/* iconst_1 */
/* putfield b5 0 */
/* return */
}

public ExecuteException getException() {
/* aload_0 */
/* getfield b4 0 */
/* ifne 9a 0 */
/* new bb 0 */
/* dup */
/* ldc 12 */
/* invokespecial b7 0 */
/* athrow */
/* aload_0 */
/* getfield b4 0 */
/* areturn */
}

public int getExitValue() {
/* aload_0 */
/* getfield b4 0 */
/* ifne 9a 0 */
/* new bb 0 */
/* dup */
/* ldc 12 */
/* invokespecial b7 0 */
/* athrow */
/* aload_0 */
/* getfield b4 0 */
/* ireturn */
}

public boolean hasResult() {
/* aload_0 */
/* getfield b4 0 */
/* ireturn */
}

public void waitFor() {
/* aload_0 */
/* invokevirtual b6 0 */
/* ifne 9a 0 */
/* ldc2W 14 0 */
/* invokestatic b8 0 */
/* goto a7 ff */
/* return */
}

public void waitFor(long arg_0) {
/* invokestatic b8 0 */
/* lload_1 */
/* ladd */
/* lstore_3 */
/* aload_0 */
/* invokevirtual b6 0 */
/* ifne 9a 0 */
/* invokestatic b8 0 */
/* lload_3 */
/* lcmp */
/* ifge 9c 0 */
/* ldc2W 14 0 */
/* invokestatic b8 0 */
/* goto a7 ff */
/* return */
}

}
