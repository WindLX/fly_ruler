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
namespace CoreOutput
{

  /// <summary>Holder for reflection information generated from core_output.proto</summary>
  public static partial class CoreOutputReflection
  {

    #region Descriptor
    /// <summary>File descriptor for core_output.proto</summary>
    public static pbr::FileDescriptor Descriptor
    {
      get { return descriptor; }
    }
    private static pbr::FileDescriptor descriptor;

    static CoreOutputReflection()
    {
      byte[] descriptorData = global::System.Convert.FromBase64String(
          string.Concat(
            "ChFjb3JlX291dHB1dC5wcm90bxILY29yZV9vdXRwdXQaCGlkLnByb3RvGgtz",
            "dGF0ZS5wcm90bxoNY29udHJvbC5wcm90bxoSc3RhdGVfZXh0ZW5kLnByb3Rv",
            "IjsKEVBsYW5lTWVzc2FnZUdyb3VwEiYKA21zZxgBIAMoCzIZLmNvcmVfb3V0",
            "cHV0LlBsYW5lTWVzc2FnZSJZCgxQbGFuZU1lc3NhZ2USEgoCaWQYASABKAsy",
            "Bi5pZC5JZBIMCgR0aW1lGAIgASgBEicKBm91dHB1dBgDIAEoCzIXLmNvcmVf",
            "b3V0cHV0LkNvcmVPdXRwdXQifQoKQ29yZU91dHB1dBIbCgVzdGF0ZRgBIAEo",
            "CzIMLnN0YXRlLlN0YXRlEiEKB2NvbnRyb2wYAiABKAsyEC5jb250cm9sLkNv",
            "bnRyb2wSLwoMc3RhdGVfZXh0ZW5kGAQgASgLMhkuc3RhdGVfZXh0ZW5kLlN0",
            "YXRlRXh0ZW5kYgZwcm90bzM="));
      descriptor = pbr::FileDescriptor.FromGeneratedCode(descriptorData,
          new pbr::FileDescriptor[] { global::Id.IdReflection.Descriptor, global::State.StateReflection.Descriptor, global::Control.ControlReflection.Descriptor, global::StateExtend.StateExtendReflection.Descriptor, },
          new pbr::GeneratedClrTypeInfo(null, null, new pbr::GeneratedClrTypeInfo[] {
            new pbr::GeneratedClrTypeInfo(typeof(global::CoreOutput.PlaneMessageGroup), global::CoreOutput.PlaneMessageGroup.Parser, new[]{ "Msg" }, null, null, null, null),
            new pbr::GeneratedClrTypeInfo(typeof(global::CoreOutput.PlaneMessage), global::CoreOutput.PlaneMessage.Parser, new[]{ "Id", "Time", "Output" }, null, null, null, null),
            new pbr::GeneratedClrTypeInfo(typeof(global::CoreOutput.CoreOutput), global::CoreOutput.CoreOutput.Parser, new[]{ "State", "Control", "StateExtend" }, null, null, null, null)
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
    public static pbr::MessageDescriptor Descriptor
    {
      get { return global::CoreOutput.CoreOutputReflection.Descriptor.MessageTypes[0]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor
    {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public PlaneMessageGroup()
    {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public PlaneMessageGroup(PlaneMessageGroup other) : this()
    {
      msg_ = other.msg_.Clone();
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public PlaneMessageGroup Clone()
    {
      return new PlaneMessageGroup(this);
    }

    /// <summary>Field number for the "msg" field.</summary>
    public const int MsgFieldNumber = 1;
    private static readonly pb::FieldCodec<global::CoreOutput.PlaneMessage> _repeated_msg_codec
        = pb::FieldCodec.ForMessage(10, global::CoreOutput.PlaneMessage.Parser);
    private readonly pbc::RepeatedField<global::CoreOutput.PlaneMessage> msg_ = new pbc::RepeatedField<global::CoreOutput.PlaneMessage>();
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public pbc::RepeatedField<global::CoreOutput.PlaneMessage> Msg
    {
      get { return msg_; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other)
    {
      return Equals(other as PlaneMessageGroup);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(PlaneMessageGroup other)
    {
      if (ReferenceEquals(other, null))
      {
        return false;
      }
      if (ReferenceEquals(other, this))
      {
        return true;
      }
      if (!msg_.Equals(other.msg_)) return false;
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode()
    {
      int hash = 1;
      hash ^= msg_.GetHashCode();
      if (_unknownFields != null)
      {
        hash ^= _unknownFields.GetHashCode();
      }
      return hash;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override string ToString()
    {
      return pb::JsonFormatter.ToDiagnosticString(this);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void WriteTo(pb::CodedOutputStream output)
    {
#if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      output.WriteRawMessage(this);
#else
      msg_.WriteTo(output, _repeated_msg_codec);
      if (_unknownFields != null) {
        _unknownFields.WriteTo(output);
      }
#endif
    }

#if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalWriteTo(ref pb::WriteContext output)
    {
      msg_.WriteTo(ref output, _repeated_msg_codec);
      if (_unknownFields != null)
      {
        _unknownFields.WriteTo(ref output);
      }
    }
#endif

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int CalculateSize()
    {
      int size = 0;
      size += msg_.CalculateSize(_repeated_msg_codec);
      if (_unknownFields != null)
      {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(PlaneMessageGroup other)
    {
      if (other == null)
      {
        return;
      }
      msg_.Add(other.msg_);
      _unknownFields = pb::UnknownFieldSet.MergeFrom(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(pb::CodedInputStream input)
    {
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
            msg_.AddEntriesFrom(input, _repeated_msg_codec);
            break;
          }
        }
      }
#endif
    }

#if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalMergeFrom(ref pb::ParseContext input)
    {
      uint tag;
      while ((tag = input.ReadTag()) != 0)
      {
        switch (tag)
        {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, ref input);
            break;
          case 10:
            {
              msg_.AddEntriesFrom(ref input, _repeated_msg_codec);
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
    public static pbr::MessageDescriptor Descriptor
    {
      get { return global::CoreOutput.CoreOutputReflection.Descriptor.MessageTypes[1]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor
    {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public PlaneMessage()
    {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public PlaneMessage(PlaneMessage other) : this()
    {
      id_ = other.id_ != null ? other.id_.Clone() : null;
      time_ = other.time_;
      output_ = other.output_ != null ? other.output_.Clone() : null;
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public PlaneMessage Clone()
    {
      return new PlaneMessage(this);
    }

    /// <summary>Field number for the "id" field.</summary>
    public const int IdFieldNumber = 1;
    private global::Id.Id id_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public global::Id.Id Id
    {
      get { return id_; }
      set
      {
        id_ = value;
      }
    }

    /// <summary>Field number for the "time" field.</summary>
    public const int TimeFieldNumber = 2;
    private double time_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public double Time
    {
      get { return time_; }
      set
      {
        time_ = value;
      }
    }

    /// <summary>Field number for the "output" field.</summary>
    public const int OutputFieldNumber = 3;
    private global::CoreOutput.CoreOutput output_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public global::CoreOutput.CoreOutput Output
    {
      get { return output_; }
      set
      {
        output_ = value;
      }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other)
    {
      return Equals(other as PlaneMessage);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(PlaneMessage other)
    {
      if (ReferenceEquals(other, null))
      {
        return false;
      }
      if (ReferenceEquals(other, this))
      {
        return true;
      }
      if (!object.Equals(Id, other.Id)) return false;
      if (!pbc::ProtobufEqualityComparers.BitwiseDoubleEqualityComparer.Equals(Time, other.Time)) return false;
      if (!object.Equals(Output, other.Output)) return false;
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode()
    {
      int hash = 1;
      if (id_ != null) hash ^= Id.GetHashCode();
      if (Time != 0D) hash ^= pbc::ProtobufEqualityComparers.BitwiseDoubleEqualityComparer.GetHashCode(Time);
      if (output_ != null) hash ^= Output.GetHashCode();
      if (_unknownFields != null)
      {
        hash ^= _unknownFields.GetHashCode();
      }
      return hash;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override string ToString()
    {
      return pb::JsonFormatter.ToDiagnosticString(this);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void WriteTo(pb::CodedOutputStream output)
    {
#if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      output.WriteRawMessage(this);
#else
      if (id_ != null) {
        output.WriteRawTag(10);
        output.WriteMessage(Id);
      }
      if (Time != 0D) {
        output.WriteRawTag(17);
        output.WriteDouble(Time);
      }
      if (output_ != null) {
        output.WriteRawTag(26);
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
    void pb::IBufferMessage.InternalWriteTo(ref pb::WriteContext output)
    {
      if (id_ != null)
      {
        output.WriteRawTag(10);
        output.WriteMessage(Id);
      }
      if (Time != 0D)
      {
        output.WriteRawTag(17);
        output.WriteDouble(Time);
      }
      if (output_ != null)
      {
        output.WriteRawTag(26);
        output.WriteMessage(Output);
      }
      if (_unknownFields != null)
      {
        _unknownFields.WriteTo(ref output);
      }
    }
#endif

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int CalculateSize()
    {
      int size = 0;
      if (id_ != null)
      {
        size += 1 + pb::CodedOutputStream.ComputeMessageSize(Id);
      }
      if (Time != 0D)
      {
        size += 1 + 8;
      }
      if (output_ != null)
      {
        size += 1 + pb::CodedOutputStream.ComputeMessageSize(Output);
      }
      if (_unknownFields != null)
      {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(PlaneMessage other)
    {
      if (other == null)
      {
        return;
      }
      if (other.id_ != null)
      {
        if (id_ == null)
        {
          Id = new global::Id.Id();
        }
        Id.MergeFrom(other.Id);
      }
      if (other.Time != 0D)
      {
        Time = other.Time;
      }
      if (other.output_ != null)
      {
        if (output_ == null)
        {
          Output = new global::CoreOutput.CoreOutput();
        }
        Output.MergeFrom(other.Output);
      }
      _unknownFields = pb::UnknownFieldSet.MergeFrom(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(pb::CodedInputStream input)
    {
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
            if (id_ == null) {
              Id = new global::Id.Id();
            }
            input.ReadMessage(Id);
            break;
          }
          case 17: {
            Time = input.ReadDouble();
            break;
          }
          case 26: {
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
    void pb::IBufferMessage.InternalMergeFrom(ref pb::ParseContext input)
    {
      uint tag;
      while ((tag = input.ReadTag()) != 0)
      {
        switch (tag)
        {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, ref input);
            break;
          case 10:
            {
              if (id_ == null)
              {
                Id = new global::Id.Id();
              }
              input.ReadMessage(Id);
              break;
            }
          case 17:
            {
              Time = input.ReadDouble();
              break;
            }
          case 26:
            {
              if (output_ == null)
              {
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
    public static pbr::MessageDescriptor Descriptor
    {
      get { return global::CoreOutput.CoreOutputReflection.Descriptor.MessageTypes[2]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor
    {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public CoreOutput()
    {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public CoreOutput(CoreOutput other) : this()
    {
      state_ = other.state_ != null ? other.state_.Clone() : null;
      control_ = other.control_ != null ? other.control_.Clone() : null;
      stateExtend_ = other.stateExtend_ != null ? other.stateExtend_.Clone() : null;
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public CoreOutput Clone()
    {
      return new CoreOutput(this);
    }

    /// <summary>Field number for the "state" field.</summary>
    public const int StateFieldNumber = 1;
    private global::State.State state_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public global::State.State State
    {
      get { return state_; }
      set
      {
        state_ = value;
      }
    }

    /// <summary>Field number for the "control" field.</summary>
    public const int ControlFieldNumber = 2;
    private global::Control.Control control_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public global::Control.Control Control
    {
      get { return control_; }
      set
      {
        control_ = value;
      }
    }

    /// <summary>Field number for the "state_extend" field.</summary>
    public const int StateExtendFieldNumber = 4;
    private global::StateExtend.StateExtend stateExtend_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public global::StateExtend.StateExtend StateExtend
    {
      get { return stateExtend_; }
      set
      {
        stateExtend_ = value;
      }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other)
    {
      return Equals(other as CoreOutput);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(CoreOutput other)
    {
      if (ReferenceEquals(other, null))
      {
        return false;
      }
      if (ReferenceEquals(other, this))
      {
        return true;
      }
      if (!object.Equals(State, other.State)) return false;
      if (!object.Equals(Control, other.Control)) return false;
      if (!object.Equals(StateExtend, other.StateExtend)) return false;
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode()
    {
      int hash = 1;
      if (state_ != null) hash ^= State.GetHashCode();
      if (control_ != null) hash ^= Control.GetHashCode();
      if (stateExtend_ != null) hash ^= StateExtend.GetHashCode();
      if (_unknownFields != null)
      {
        hash ^= _unknownFields.GetHashCode();
      }
      return hash;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override string ToString()
    {
      return pb::JsonFormatter.ToDiagnosticString(this);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void WriteTo(pb::CodedOutputStream output)
    {
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
    void pb::IBufferMessage.InternalWriteTo(ref pb::WriteContext output)
    {
      if (state_ != null)
      {
        output.WriteRawTag(10);
        output.WriteMessage(State);
      }
      if (control_ != null)
      {
        output.WriteRawTag(18);
        output.WriteMessage(Control);
      }
      if (stateExtend_ != null)
      {
        output.WriteRawTag(34);
        output.WriteMessage(StateExtend);
      }
      if (_unknownFields != null)
      {
        _unknownFields.WriteTo(ref output);
      }
    }
#endif

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int CalculateSize()
    {
      int size = 0;
      if (state_ != null)
      {
        size += 1 + pb::CodedOutputStream.ComputeMessageSize(State);
      }
      if (control_ != null)
      {
        size += 1 + pb::CodedOutputStream.ComputeMessageSize(Control);
      }
      if (stateExtend_ != null)
      {
        size += 1 + pb::CodedOutputStream.ComputeMessageSize(StateExtend);
      }
      if (_unknownFields != null)
      {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(CoreOutput other)
    {
      if (other == null)
      {
        return;
      }
      if (other.state_ != null)
      {
        if (state_ == null)
        {
          State = new global::State.State();
        }
        State.MergeFrom(other.State);
      }
      if (other.control_ != null)
      {
        if (control_ == null)
        {
          Control = new global::Control.Control();
        }
        Control.MergeFrom(other.Control);
      }
      if (other.stateExtend_ != null)
      {
        if (stateExtend_ == null)
        {
          StateExtend = new global::StateExtend.StateExtend();
        }
        StateExtend.MergeFrom(other.StateExtend);
      }
      _unknownFields = pb::UnknownFieldSet.MergeFrom(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(pb::CodedInputStream input)
    {
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
    void pb::IBufferMessage.InternalMergeFrom(ref pb::ParseContext input)
    {
      uint tag;
      while ((tag = input.ReadTag()) != 0)
      {
        switch (tag)
        {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, ref input);
            break;
          case 10:
            {
              if (state_ == null)
              {
                State = new global::State.State();
              }
              input.ReadMessage(State);
              break;
            }
          case 18:
            {
              if (control_ == null)
              {
                Control = new global::Control.Control();
              }
              input.ReadMessage(Control);
              break;
            }
          case 34:
            {
              if (stateExtend_ == null)
              {
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
