package org.enso.interpreter.node.controlflow.caseexpr;

import com.oracle.truffle.api.nodes.ControlFlowException;
import com.oracle.truffle.api.nodes.NodeInfo;

/** This exception is used to signal when a certain branch in a case expression has been taken. */
@NodeInfo(shortName = "BranchSelect", description = "Signals that a case branch has been selected")
public class BranchSelectedException extends ControlFlowException {
  private final BranchResult result;

  /**
   * Creates a new exception instance. The result is wrapped in `CaseResult` to indiciate if the
   * result represents a failed nested branch or actually a complete and successful execution of a
   * (potentially nested) branch.
   *
   * @param result the result of executing the branch this is thrown from
   */
  public BranchSelectedException(BranchResult result) {
    this.result = result;
  }

  /**
   * Gets the result of executing the case branch in question.
   *
   * @return the result of executing the case branch from which this is thrown
   */
  public BranchResult getBranchResult() {
    return result;
  }
}
