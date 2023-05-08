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
    super();
    // aload_0
    // iconst_0
    // putfield 0xB5 0x0
    // aload_0
    // ldc 0x12
    // putfield 0xB5 0x0
    // return
    }
  
  public void onProcessComplete(int arg_0) {
    // aload_0
    // iload_1
    // putfield 0xB5 0x0
    // aload_0
    // aconst_null
    // putfield 0xB5 0x0
    // aload_0
    // iconst_1
    // putfield 0xB5 0x0
    // return
    }
  
  public void onProcessFailed(ExecuteException arg_0) {
    // aload_0
    // aload_1
    // invokevirtual 0xB6 0x0
    // putfield 0xB5 0x0
    // aload_0
    // aload_1
    // putfield 0xB5 0x0
    // aload_0
    // iconst_1
    // putfield 0xB5 0x0
    // return
    }
  
  public ExecuteException getException() {
    // aload_0
    // getfield 0xB4 0x0
    // ifne 0x9A 0x0
    // new 0xBB 0x0
    // dup
    // ldc 0x12
    // invokespecial 0xB7 0x0
    // athrow
    // aload_0
    // getfield 0xB4 0x0
    // areturn
    }
  
  public int getExitValue() {
    // aload_0
    // getfield 0xB4 0x0
    // ifne 0x9A 0x0
    // new 0xBB 0x0
    // dup
    // ldc 0x12
    // invokespecial 0xB7 0x0
    // athrow
    // aload_0
    // getfield 0xB4 0x0
    // ireturn
    }
  
  public boolean hasResult() {
    // aload_0
    // getfield 0xB4 0x0
    // ireturn
    }
  
  public void waitFor() {
    // aload_0
    // invokevirtual 0xB6 0x0
    // ifne 0x9A 0x0
    // ldc2W 0x14 0x0
    // invokestatic 0xB8 0x0
    // goto 0xA7 0xFF
    // return
    }
  
  public void waitFor(long arg_0) {
    // invokestatic 0xB8 0x0
    // lload_1
    // ladd
    // lstore_3
    // aload_0
    // invokevirtual 0xB6 0x0
    // ifne 0x9A 0x0
    // invokestatic 0xB8 0x0
    // lload_3
    // lcmp
    // ifge 0x9C 0x0
    // ldc2W 0x14 0x0
    // invokestatic 0xB8 0x0
    // goto 0xA7 0xFF
    // return
    }
  
  }
