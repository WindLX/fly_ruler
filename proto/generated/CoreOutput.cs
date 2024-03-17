// <auto-generated>
//     Generated by the protocol buffer compiler.  DO NOT EDIT!
//     source: core_output.proto
// </auto-generated>
#pragma warning disable 1591, 0612, 3021, 8981
#region Designer generated code

using pb = global::Google.Protobuf;
using pbc = global::Google.Protobuf.Collections;
using pbr = global::Google.Protobuf.Reflection;
using scg = global::System.Collections.Generic;
namespace CoreOutput {

  /// <summary>Holder for reflection information generated from core_output.proto</summary>
  public static partial class CoreOutputReflection {

    #region Descriptor
    /// <summary>File descriptor for core_output.proto</summary>
    public static pbr::FileDescriptor Descriptor {
      get { return descriptor; }
    }
    private static pbr::FileDescriptor descriptor;

    static CoreOutputReflection() {
      byte[] descriptorData = global::System.Convert.FromBase64String(
          string.Concat(
            "ChFjb3JlX291dHB1dC5wcm90bxILY29yZV9vdXRwdXQaC3N0YXRlLnByb3Rv",
            "Gg1jb250cm9sLnByb3RvGhJzdGF0ZV9leHRlbmQucHJvdG8iTgoRUGxhbmVN",
            "ZXNzYWdlR3JvdXASDAoEdGltZRgBIAEoARIrCgh2aWV3X21zZxgCIAMoCzIZ",
            "LmNvcmVfb3V0cHV0LlBsYW5lTWVzc2FnZSJDCgxQbGFuZU1lc3NhZ2USCgoC",
            "aWQYASABKAkSJwoGb3V0cHV0GAIgASgLMhcuY29yZV9vdXRwdXQuQ29yZU91",
            "dHB1dCKMAQoKQ29yZU91dHB1dBIbCgVzdGF0ZRgBIAEoCzIMLnN0YXRlLlN0",
            "YXRlEiEKB2NvbnRyb2wYAiABKAsyEC5jb250cm9sLkNvbnRyb2wSDQoFZF9s",
            "ZWYYAyABKAESLwoMc3RhdGVfZXh0ZW5kGAQgASgLMhkuc3RhdGVfZXh0ZW5k",
            "LlN0YXRlRXh0ZW5kYgZwcm90bzM="));
      descriptor = pbr::FileDescriptor.FromGeneratedCode(descriptorData,
          new pbr::FileDescriptor[] { global::State.StateReflection.Descriptor, global::Control.ControlReflection.Descriptor, global::StateExtend.StateExtendReflection.Descriptor, },
          new pbr::GeneratedClrTypeInfo(null, null, new pbr::GeneratedClrTypeInfo[] {
            new pbr::GeneratedClrTypeInfo(typeof(global::CoreOutput.PlaneMessageGroup), global::CoreOutput.PlaneMessageGroup.Parser, new[]{ "Time", "ViewMsg" }, null, null, null, null),
            new pbr::GeneratedClrTypeInfo(typeof(global::CoreOutput.PlaneMessage), global::CoreOutput.PlaneMessage.Parser, new[]{ "Id", "Output" }, null, null, null, null),
            new pbr::GeneratedClrTypeInfo(typeof(global::CoreOutput.CoreOutput), global::CoreOutput.CoreOutput.Parser, new[]{ "State", "Control", "DLef", "StateExtend" }, null, null, null, null)
          }));
    }
    #endregion

  }
  #region Messages
  [global::System.Diagnostics.DebuggerDisplayAttribute("{ToString(),nq}")]
  public sealed partial class PlaneMessageGroup : pb::IMessage<PlaneMessageGroup>
  #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      , pb::IBufferMessage
  #endif
  {
    private static readonly pb::MessageParser<PlaneMessageGroup> _parser = new pb::MessageParser<PlaneMessageGroup>(() => new PlaneMessageGroup());
    private pb::UnknownFieldSet _unknownFields;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pb::MessageParser<PlaneMessageGroup> Parser { get { return _parser; } }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pbr::MessageDescriptor Descriptor {
      get { return global::CoreOutput.CoreOutputReflection.Descriptor.MessageTypes[0]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public PlaneMessageGroup() {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public PlaneMessageGroup(PlaneMessageGroup other) : this() {
      time_ = other.time_;
      viewMsg_ = other.viewMsg_.Clone();
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public PlaneMessageGroup Clone() {
      return new PlaneMessageGroup(this);
    }

    /// <summary>Field number for the "time" field.</summary>
    public const int TimeFieldNumber = 1;
    private double time_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public double Time {
      get { return time_; }
      set {
        time_ = value;
      }
    }

    /// <summary>Field number for the "view_msg" field.</summary>
    public const int ViewMsgFieldNumber = 2;
    private static readonly pb::FieldCodec<global::CoreOutput.PlaneMessage> _repeated_viewMsg_codec
        = pb::FieldCodec.ForMessage(18, global::CoreOutput.PlaneMessage.Parser);
    private readonly pbc::RepeatedField<global::CoreOutput.PlaneMessage> viewMsg_ = new pbc::RepeatedField<global::CoreOutput.PlaneMessage>();
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public pbc::RepeatedField<global::CoreOutput.PlaneMessage> ViewMsg {
      get { return viewMsg_; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other) {
      return Equals(other as PlaneMessageGroup);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(PlaneMessageGroup other) {
      if (ReferenceEquals(other, null)) {
        return false;
      }
      if (ReferenceEquals(other, this)) {
        return true;
      }
      if (!pbc::ProtobufEqualityComparers.BitwiseDoubleEqualityComparer.Equals(Time, other.Time)) return false;
      if(!viewMsg_.Equals(other.viewMsg_)) return false;
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode() {
      int hash = 1;
      if (Time != 0D) hash ^= pbc::ProtobufEqualityComparers.BitwiseDoubleEqualityComparer.GetHashCode(Time);
      hash ^= viewMsg_.GetHashCode();
      if (_unknownFields != null) {
        hash ^= _unknownFields.GetHashCode();
      }
      return hash;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override string ToString() {
      return pb::JsonFormatter.ToDiagnosticString(this);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void WriteTo(pb::CodedOutputStream output) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      output.WriteRawMessage(this);
    #else
      if (Time != 0D) {
        output.WriteRawTag(9);
        output.WriteDouble(Time);
      }
      viewMsg_.WriteTo(output, _repeated_viewMsg_codec);
      if (_unknownFields != null) {
        _unknownFields.WriteTo(output);
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalWriteTo(ref pb::WriteContext output) {
      if (Time != 0D) {
        output.WriteRawTag(9);
        output.WriteDouble(Time);
      }
      viewMsg_.WriteTo(ref output, _repeated_viewMsg_codec);
      if (_unknownFields != null) {
        _unknownFields.WriteTo(ref output);
      }
    }
    #endif

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int CalculateSize() {
      int size = 0;
      if (Time != 0D) {
        size += 1 + 8;
      }
      size += viewMsg_.CalculateSize(_repeated_viewMsg_codec);
      if (_unknownFields != null) {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(PlaneMessageGroup other) {
      if (other == null) {
        return;
      }
      if (other.Time != 0D) {
        Time = other.Time;
      }
      viewMsg_.Add(other.viewMsg_);
      _unknownFields = pb::UnknownFieldSet.MergeFrom(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(pb::CodedInputStream input) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      input.ReadRawMessage(this);
    #else
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
        switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, input);
            break;
          case 9: {
            Time = input.ReadDouble();
            break;
          }
          case 18: {
            viewMsg_.AddEntriesFrom(input, _repeated_viewMsg_codec);
            break;
          }
        }
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalMergeFrom(ref pb::ParseContext input) {
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
        switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, ref input);
            break;
          case 9: {
            Time = input.ReadDouble();
            break;
          }
          case 18: {
            viewMsg_.AddEntriesFrom(ref input, _repeated_viewMsg_codec);
            break;
          }
        }
      }
    }
    #endif

  }

  [global::System.Diagnostics.DebuggerDisplayAttribute("{ToString(),nq}")]
  public sealed partial class PlaneMessage : pb::IMessage<PlaneMessage>
  #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      , pb::IBufferMessage
  #endif
  {
    private static readonly pb::MessageParser<PlaneMessage> _parser = new pb::MessageParser<PlaneMessage>(() => new PlaneMessage());
    private pb::UnknownFieldSet _unknownFields;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pb::MessageParser<PlaneMessage> Parser { get { return _parser; } }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pbr::MessageDescriptor Descriptor {
      get { return global::CoreOutput.CoreOutputReflection.Descriptor.MessageTypes[1]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public PlaneMessage() {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public PlaneMessage(PlaneMessage other) : this() {
      id_ = other.id_;
      output_ = other.output_ != null ? other.output_.Clone() : null;
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public PlaneMessage Clone() {
      return new PlaneMessage(this);
    }

    /// <summary>Field number for the "id" field.</summary>
    public const int IdFieldNumber = 1;
    private string id_ = "";
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public string Id {
      get { return id_; }
      set {
        id_ = pb::ProtoPreconditions.CheckNotNull(value, "value");
      }
    }

    /// <summary>Field number for the "output" field.</summary>
    public const int OutputFieldNumber = 2;
    private global::CoreOutput.CoreOutput output_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public global::CoreOutput.CoreOutput Output {
      get { return output_; }
      set {
        output_ = value;
      }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other) {
      return Equals(other as PlaneMessage);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(PlaneMessage other) {
      if (ReferenceEquals(other, null)) {
        return false;
      }
      if (ReferenceEquals(other, this)) {
        return true;
      }
      if (Id != other.Id) return false;
      if (!object.Equals(Output, other.Output)) return false;
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode() {
      int hash = 1;
      if (Id.Length != 0) hash ^= Id.GetHashCode();
      if (output_ != null) hash ^= Output.GetHashCode();
      if (_unknownFields != null) {
        hash ^= _unknownFields.GetHashCode();
      }
      return hash;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override string ToString() {
      return pb::JsonFormatter.ToDiagnosticString(this);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void WriteTo(pb::CodedOutputStream output) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      output.WriteRawMessage(this);
    #else
      if (Id.Length != 0) {
        output.WriteRawTag(10);
        output.WriteString(Id);
      }
      if (output_ != null) {
        output.WriteRawTag(18);
        output.WriteMessage(Output);
      }
      if (_unknownFields != null) {
        _unknownFields.WriteTo(output);
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalWriteTo(ref pb::WriteContext output) {
      if (Id.Length != 0) {
        output.WriteRawTag(10);
        output.WriteString(Id);
      }
      if (output_ != null) {
        output.WriteRawTag(18);
        output.WriteMessage(Output);
      }
      if (_unknownFields != null) {
        _unknownFields.WriteTo(ref output);
      }
    }
    #endif

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int CalculateSize() {
      int size = 0;
      if (Id.Length != 0) {
        size += 1 + pb::CodedOutputStream.ComputeStringSize(Id);
      }
      if (output_ != null) {
        size += 1 + pb::CodedOutputStream.ComputeMessageSize(Output);
      }
      if (_unknownFields != null) {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(PlaneMessage other) {
      if (other == null) {
        return;
      }
      if (other.Id.Length != 0) {
        Id = other.Id;
      }
      if (other.output_ != null) {
        if (output_ == null) {
          Output = new global::CoreOutput.CoreOutput();
        }
        Output.MergeFrom(other.Output);
      }
      _unknownFields = pb::UnknownFieldSet.MergeFrom(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(pb::CodedInputStream input) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      input.ReadRawMessage(this);
    #else
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
        switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, input);
            break;
          case 10: {
            Id = input.ReadString();
            break;
          }
          case 18: {
            if (output_ == null) {
              Output = new global::CoreOutput.CoreOutput();
            }
            input.ReadMessage(Output);
            break;
          }
        }
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalMergeFrom(ref pb::ParseContext input) {
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
        switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, ref input);
            break;
          case 10: {
            Id = input.ReadString();
            break;
          }
          case 18: {
            if (output_ == null) {
              Output = new global::CoreOutput.CoreOutput();
            }
            input.ReadMessage(Output);
            break;
          }
        }
      }
    }
    #endif

  }

  [global::System.Diagnostics.DebuggerDisplayAttribute("{ToString(),nq}")]
  public sealed partial class CoreOutput : pb::IMessage<CoreOutput>
  #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      , pb::IBufferMessage
  #endif
  {
    private static readonly pb::MessageParser<CoreOutput> _parser = new pb::MessageParser<CoreOutput>(() => new CoreOutput());
    private pb::UnknownFieldSet _unknownFields;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pb::MessageParser<CoreOutput> Parser { get { return _parser; } }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pbr::MessageDescriptor Descriptor {
      get { return global::CoreOutput.CoreOutputReflection.Descriptor.MessageTypes[2]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public CoreOutput() {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public CoreOutput(CoreOutput other) : this() {
      state_ = other.state_ != null ? other.state_.Clone() : null;
      control_ = other.control_ != null ? other.control_.Clone() : null;
      dLef_ = other.dLef_;
      stateExtend_ = other.stateExtend_ != null ? other.stateExtend_.Clone() : null;
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public CoreOutput Clone() {
      return new CoreOutput(this);
    }

    /// <summary>Field number for the "state" field.</summary>
    public const int StateFieldNumber = 1;
    private global::State.State state_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public global::State.State State {
      get { return state_; }
      set {
        state_ = value;
      }
    }

    /// <summary>Field number for the "control" field.</summary>
    public const int ControlFieldNumber = 2;
    private global::Control.Control control_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public global::Control.Control Control {
      get { return control_; }
      set {
        control_ = value;
      }
    }

    /// <summary>Field number for the "d_lef" field.</summary>
    public const int DLefFieldNumber = 3;
    private double dLef_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public double DLef {
      get { return dLef_; }
      set {
        dLef_ = value;
      }
    }

    /// <summary>Field number for the "state_extend" field.</summary>
    public const int StateExtendFieldNumber = 4;
    private global::StateExtend.StateExtend stateExtend_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public global::StateExtend.StateExtend StateExtend {
      get { return stateExtend_; }
      set {
        stateExtend_ = value;
      }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other) {
      return Equals(other as CoreOutput);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(CoreOutput other) {
      if (ReferenceEquals(other, null)) {
        return false;
      }
      if (ReferenceEquals(other, this)) {
        return true;
      }
      if (!object.Equals(State, other.State)) return false;
      if (!object.Equals(Control, other.Control)) return false;
      if (!pbc::ProtobufEqualityComparers.BitwiseDoubleEqualityComparer.Equals(DLef, other.DLef)) return false;
      if (!object.Equals(StateExtend, other.StateExtend)) return false;
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode() {
      int hash = 1;
      if (state_ != null) hash ^= State.GetHashCode();
      if (control_ != null) hash ^= Control.GetHashCode();
      if (DLef != 0D) hash ^= pbc::ProtobufEqualityComparers.BitwiseDoubleEqualityComparer.GetHashCode(DLef);
      if (stateExtend_ != null) hash ^= StateExtend.GetHashCode();
      if (_unknownFields != null) {
        hash ^= _unknownFields.GetHashCode();
      }
      return hash;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override string ToString() {
      return pb::JsonFormatter.ToDiagnosticString(this);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void WriteTo(pb::CodedOutputStream output) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      output.WriteRawMessage(this);
    #else
      if (state_ != null) {
        output.WriteRawTag(10);
        output.WriteMessage(State);
      }
      if (control_ != null) {
        output.WriteRawTag(18);
        output.WriteMessage(Control);
      }
      if (DLef != 0D) {
        output.WriteRawTag(25);
        output.WriteDouble(DLef);
      }
      if (stateExtend_ != null) {
        output.WriteRawTag(34);
        output.WriteMessage(StateExtend);
      }
      if (_unknownFields != null) {
        _unknownFields.WriteTo(output);
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalWriteTo(ref pb::WriteContext output) {
      if (state_ != null) {
        output.WriteRawTag(10);
        output.WriteMessage(State);
      }
      if (control_ != null) {
        output.WriteRawTag(18);
        output.WriteMessage(Control);
      }
      if (DLef != 0D) {
        output.WriteRawTag(25);
        output.WriteDouble(DLef);
      }
      if (stateExtend_ != null) {
        output.WriteRawTag(34);
        output.WriteMessage(StateExtend);
      }
      if (_unknownFields != null) {
        _unknownFields.WriteTo(ref output);
      }
    }
    #endif

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int CalculateSize() {
      int size = 0;
      if (state_ != null) {
        size += 1 + pb::CodedOutputStream.ComputeMessageSize(State);
      }
      if (control_ != null) {
        size += 1 + pb::CodedOutputStream.ComputeMessageSize(Control);
      }
      if (DLef != 0D) {
        size += 1 + 8;
      }
      if (stateExtend_ != null) {
        size += 1 + pb::CodedOutputStream.ComputeMessageSize(StateExtend);
      }
      if (_unknownFields != null) {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(CoreOutput other) {
      if (other == null) {
        return;
      }
      if (other.state_ != null) {
        if (state_ == null) {
          State = new global::State.State();
        }
        State.MergeFrom(other.State);
      }
      if (other.control_ != null) {
        if (control_ == null) {
          Control = new global::Control.Control();
        }
        Control.MergeFrom(other.Control);
      }
      if (other.DLef != 0D) {
        DLef = other.DLef;
      }
      if (other.stateExtend_ != null) {
        if (stateExtend_ == null) {
          StateExtend = new global::StateExtend.StateExtend();
        }
        StateExtend.MergeFrom(other.StateExtend);
      }
      _unknownFields = pb::UnknownFieldSet.MergeFrom(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(pb::CodedInputStream input) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      input.ReadRawMessage(this);
    #else
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
        switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, input);
            break;
          case 10: {
            if (state_ == null) {
              State = new global::State.State();
            }
            input.ReadMessage(State);
            break;
          }
          case 18: {
            if (control_ == null) {
              Control = new global::Control.Control();
            }
            input.ReadMessage(Control);
            break;
          }
          case 25: {
            DLef = input.ReadDouble();
            break;
          }
          case 34: {
            if (stateExtend_ == null) {
              StateExtend = new global::StateExtend.StateExtend();
            }
            input.ReadMessage(StateExtend);
            break;
          }
        }
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalMergeFrom(ref pb::ParseContext input) {
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
        switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, ref input);
            break;
          case 10: {
            if (state_ == null) {
              State = new global::State.State();
            }
            input.ReadMessage(State);
            break;
          }
          case 18: {
            if (control_ == null) {
              Control = new global::Control.Control();
            }
            input.ReadMessage(Control);
            break;
          }
          case 25: {
            DLef = input.ReadDouble();
            break;
          }
          case 34: {
            if (stateExtend_ == null) {
              StateExtend = new global::StateExtend.StateExtend();
            }
            input.ReadMessage(StateExtend);
            break;
          }
        }
      }
    }
    #endif

  }

  #endregion

}

#endregion Designer generated code